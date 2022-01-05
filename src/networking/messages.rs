use serde::{Deserialize, Serialize};
use itertools::Itertools;
use std::sync::Mutex;
use std::collections::HashSet;
use once_cell::sync::Lazy;

use bevy::prelude::*;
use bevy_tilemap::Tilemap;

use bevy_spicy_networking::{
    ConnectionId,
    ClientMessage,
    NetworkMessage,
    ServerMessage,
    NetworkData,
    AppNetworkClientMessage,
    AppNetworkServerMessage,
};

use crate::generation::Factors;
use crate::generation::{Unit,id::*};
use crate::objects::Point;

use crate::systems::network::NetworkState;
use crate::systems::gui::GuiState;
use crate::state::State;
use crate::resources::Label;
use crate::state::traits::*;

macro_rules! name {
    ( $n: ident ) => {
        concat!("warfare:",stringify!($n))
    }
}

macro_rules! register {
    ( $a: ident, $n: ty ) => {
        $a.listen_for_client_message::<$n>();
        $a.listen_for_server_message::<$n>();
    }
}

macro_rules! message {
    ( $d: ident, $n: ident ( $v: ty )) => {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct $n($v);

        impl $n {
            pub fn new(v: $v) -> Self {
                Self(v)
            }

            pub fn value(&self) -> &$v {
                &self.0
            }

            pub fn take(self) -> $v {
                self.0
            }
        }

        #[typetag::serde]
        impl NetworkMessage for $n {}

        impl ServerMessage for $n {
            const NAME: &'static str = name!($n);
        }
        
        impl ClientMessage for $n {
            const NAME: &'static str = name!($n);
        }

        impl Message for $n {
            fn target(&self) -> Option<PlayerId> {
                self.0.header.target
            }

            fn sender(&self) -> PlayerId {
                self.0.header.sender
            }

            fn name(&self) -> String {
                self.0.header.name.clone()
            }

            fn id(&self) -> Option<MessageId> {
                self.0.header.id.clone()
            }

            fn set_id(&mut self, id: MessageId) {
                self.0.header.id = Some(id);
            }

            fn data(&self) -> MessageData {
                MessageData::$d(self.0.clone())
            }
        }

        impl From<&NetworkData<$n>> for $n {
            fn from(data: &NetworkData<$n>) -> $n {
                (*data).clone()
            }
        }

        impl From<$v> for $n {
            fn from(data: $v) -> $n {
                Self::new(data)
            }
        }
    };
}

pub trait Message {

    fn target(&self) -> Option<PlayerId>;

    fn sender(&self) -> PlayerId;

    fn name(&self) -> String;

    fn id(&self) -> Option<MessageId>;

    fn set_id(&mut self, id: MessageId);

    fn data(&self) -> MessageData;
}

pub struct MessagePlugin;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderData {
    pub id: Option<MessageId>,
    pub target: Option<PlayerId>,
    pub sender: PlayerId,
    pub name: String,
}

impl HeaderData {
    pub fn new(sender: PlayerId, name: String) -> Self {
        Self {
            id:     None,
            target: None,
            sender: sender,
            name:   name,
        }
    }
    pub fn with_target(mut self, target: PlayerId) -> Self {
        self.target = Some(target);
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyData {
    pub header: HeaderData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinData {
    pub header: HeaderData,
    pub name: String,
    pub code: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfirmData {
    pub header: HeaderData,
    pub motd: String,
    pub code: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitData {
    pub header: HeaderData,
    pub unit: Unit,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MoveData {
    pub header: HeaderData,
    pub moves: Vec<(Id,Point,u8)>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatData {
    pub header: HeaderData,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerrainData {
    pub header: HeaderData,
    pub seed: u32,
    pub turn: u32,
    pub factors: Factors
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageData {
    Join(JoinData),      // player joined
    Confirm(ConfirmData),// confirm connection
    Create(UnitData),    // unit created
    Move(MoveData),      // unit moved
    Chat(ChatData),      // chat message
    Update(TerrainData), // update response
    Refresh(EmptyData),  // request update
}

message!(Join,JoinMessage(JoinData));
message!(Confirm,ConfirmMessage(ConfirmData));
message!(Create,CreateMessage(UnitData));
message!(Move,MoveMessage(MoveData));
message!(Chat,ChatMessage(ChatData));
message!(Update,UpdateMessage(TerrainData));
message!(Refresh,RefreshMessage(EmptyData));

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        register!(app,JoinMessage);
        register!(app,ConfirmMessage);
        register!(app,CreateMessage);
        register!(app,MoveMessage);
        register!(app,ChatMessage);
        register!(app,UpdateMessage);
        register!(app,RefreshMessage);
    }
}

macro_rules! require {
    ( $check: expr ) => { 
        if !$check { return; } 
    };
    ( $check: expr, $msg: expr ) => { 
        if !$check { 
            warn!($msg);
            return;
        } 
    }
}

macro_rules! require_client {
    ( $network: ident ) => { require!($network.is_client(),"Must be client") }
}

macro_rules! require_server {
    ( $network: ident ) => { require!($network.is_server(),"Must be server") }
}

macro_rules! require_other {
    ( $network: ident, $id: expr ) => { require!(($network.id() != $id),"Must be other") }
}

macro_rules! require_registered {
    ( $message: ident ) => { require!(($message.id().is_some()),"Must be registered") }
}

static MESSAGES: Lazy<Mutex<HashSet<MessageId>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn registered(id: &MessageId) -> bool {
    MESSAGES.lock()
            .expect("failed to lock registration list")
            .contains(id)
}

pub fn register(id: &MessageId) {
    MESSAGES.lock()
            .expect("failed to lock registration list")
            .insert(id.clone());
}

impl JoinMessage {
    pub fn apply(&self, network: &mut NetworkState) {
        require_server!(network);
        if let Some(conn) = network.stop_waiting(self.value().code) {
            let player = network.players.insert(
                self.sender(),
                conn,
                self.value().name.clone(),
            );
            warn!("player joined: {} ({})",player.name,player.id);
        }
    }
}

impl ConfirmMessage {
    pub fn apply(&self, network: &mut NetworkState, gui: &mut GuiState) {
        require_registered!(self);
        network.send_join_event(self.value().code);
        gui.add_message(
            "Server".into(),
            self.value().motd.clone(),
        );
    }
}

impl CreateMessage {
    pub fn apply(&self, network: &NetworkState, map: &mut Tilemap, state: &mut State) {
        require_other!(network,self.sender()); // cannot apply to self
        require_registered!(self);

        // TODO: streamline this add process
        // add new unit to tilemap and unit map if it doesn't exist
        let data = self.value();
        let mut unit = data.unit.clone();
        let point = unit.position().clone();

        *(unit.texture_mut()) = state.textures.get(Label::Unit);

        unit.insert(map);
        state.units.add(point,unit);
    }
}

impl MoveMessage {
    pub fn apply(&self, network: &NetworkState, map: &mut Tilemap, state: &mut State) {
        require_other!(network,self.sender()); // cannot apply to self
        require_registered!(self);
        let data = self.value();
        for (point,moves) in data.moves.iter().group_by(|m| m.1).into_iter() {
            let movement = moves
                .into_iter()
                .map(|m| (m.0,m.2))
                .collect();
            state.units.transfer(map,movement,point);
        }
    }
}

impl ChatMessage {
    pub fn apply(&self, network: &NetworkState, gui: &mut GuiState) {
        require_registered!(self);
        gui.add_message(
            self.name(),
            self.value().message.clone());
    }
}

impl UpdateMessage {
    pub fn apply(&self, network: &NetworkState, state: &mut State) {
        require_registered!(self);

        state.sync(self.value().clone());
    }
}