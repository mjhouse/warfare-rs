use serde::{Deserialize, Serialize};
use itertools::Itertools;
use std::sync::Mutex;
use std::collections::HashSet;
use once_cell::sync::Lazy;

use bevy::prelude::*;
use bevy_tilemap::Tilemap;

use bevy_spicy_networking::{
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
        #[derive(Serialize, Deserialize, Clone, Debug, Default)]
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

    fn id(&self) -> Option<MessageId>;

    fn set_id(&mut self, id: MessageId);

    fn data(&self) -> MessageData;
}

pub struct MessagePlugin;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HeaderData {
    pub id: Option<MessageId>,
    pub target: Option<PlayerId>,
    pub sender: PlayerId,
}

impl HeaderData {
    pub fn new(sender: PlayerId) -> Self {
        Self {
            id:     None,
            target: None,
            sender: sender,
        }
    }
    pub fn with_target(mut self, target: PlayerId) -> Self {
        self.target = Some(target);
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmptyData {
    pub header: HeaderData,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct JoinData {
    pub header: HeaderData,
    pub player: PlayerId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ConfirmData {
    pub header: HeaderData,
    pub player: PlayerId,
    pub motd: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UnitData {
    pub header: HeaderData,
    pub unit: Unit,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MoveData {
    pub header: HeaderData,
    pub moves: Vec<(Id,Point,u8)>
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ChatData {
    pub header: HeaderData,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
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
    ( $network: ident ) => { require!($network.is_client()) }
}

macro_rules! require_server {
    ( $network: ident ) => { require!($network.is_server()) }
}

macro_rules! require_other {
    ( $network: ident, $id: expr ) => { require!(($network.id() == $id)) }
}

macro_rules! require_registered {
    ( $message: ident ) => { require!(($message.id().is_some())) }
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
    pub fn apply(&self, gui: &mut GuiState) {
        gui.add_message(self.sender(),
            format!("Player {} has joined",self.value().player));
    }
}

impl ConfirmMessage {
    pub fn apply(&self, network: &mut NetworkState, gui: &mut GuiState) {
        require_other!(network,self.sender()); // cannot apply to self
        require_registered!(self);

        let data = self.value();
        network.set_id(data.player);
        network.set_motd(data.motd.clone());
        gui.add_message(self.sender(),data.motd.clone());
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

        // TODO: apply actions to moved units
        // group by point-being-moved-to and select + move all units
        let data = self.value();
        for (point,moves) in data.moves.iter().group_by(|m| m.1).into_iter() {
            let ids = moves
                .into_iter()
                .map(|m| m.0)
                .collect();
            state.units.select(&ids);
            state.units.move_selection(map,&point);
            state.units.select_none_free();
        }
    }
}

impl ChatMessage {
    pub fn apply(&self, network: &NetworkState, gui: &mut GuiState) {
        require_registered!(self);
        
        gui.add_message(
            self.sender(),
            self.value().message.clone());
    }
}

impl UpdateMessage {
    pub fn apply(&self, network: &NetworkState, state: &mut State) {
        require_registered!(self);

        state.sync(self.value().clone());
    }
}