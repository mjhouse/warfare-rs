use serde::{Deserialize, Serialize};
use itertools::Itertools;
use std::sync::Mutex;
use std::collections::HashSet;
use once_cell::sync::Lazy;
use std::collections::HashMap;

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
use crate::generation::{Unit,id::*,Change};
use crate::objects::Point;

use crate::systems::network::NetworkState;
use crate::systems::gui::GuiState;
use crate::state::State;
use crate::resources::Label;
use crate::state::traits::*;

// messages that have been rebroadcasted
static MESSAGES: Lazy<Mutex<HashSet<MessageId>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn check_applied(id: MessageId) -> bool {
    MESSAGES.lock()
            .expect("failed to lock applied list")
            .contains(&id)
}

pub fn mark_applied(id: MessageId) {
    MESSAGES.lock()
            .expect("failed to lock applied list")
            .insert(id);
}

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

    fn sender(&self) -> PlayerId;

    fn name(&self) -> String;

    fn id(&self) -> Option<MessageId>;

    fn set_id(&mut self, id: MessageId);

    fn is_registered(&self) -> bool {
        self.id().is_some()
    }

    fn is_applied(&self) -> bool {
        self.id().map(check_applied).unwrap_or(false)
    }

    fn set_applied(&self) {
        self.id().map(mark_applied);
    }

    fn data(&self) -> MessageData;
}

pub struct MessagePlugin;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderData {
    pub id: Option<MessageId>,
    pub sender: PlayerId,
    pub name: String,
}

impl HeaderData {
    pub fn new(sender: PlayerId, name: String) -> Self {
        Self {
            id:     None,
            sender: sender,
            name:   name,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyData {
    pub header: HeaderData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChangeData {
    pub header: HeaderData,
    pub changes: Vec<Change>,
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
pub struct PlayerData {
    pub id: PlayerId,
    pub name: String,
    pub order: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateData {
    pub header: HeaderData,
    pub seed: u32,
    pub turn: u32,
    pub factors: Factors,
    pub players: Vec<PlayerData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageData {
    Join(JoinData),      // player joined
    Change(ChangeData),  // apply changes to units
    Confirm(ConfirmData),// confirm connection
    Create(UnitData),    // unit created
    Move(MoveData),      // unit moved
    Chat(ChatData),      // chat message
    Update(UpdateData),  // update response
    Refresh(EmptyData),  // request update
}

message!(Join,JoinMessage(JoinData));
message!(Change,ChangeMessage(ChangeData));
message!(Confirm,ConfirmMessage(ConfirmData));
message!(Create,CreateMessage(UnitData));
message!(Move,MoveMessage(MoveData));
message!(Chat,ChatMessage(ChatData));
message!(Update,UpdateMessage(UpdateData));
message!(Refresh,RefreshMessage(EmptyData));

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        register!(app,JoinMessage);
        register!(app,ChangeMessage);
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
    ( $message: ident ) => { require!(($message.is_registered()),"Must be registered") }
}

macro_rules! require_unapplied {
    ( $message: ident ) => { require!((!$message.is_applied()),"Must be unapplied") }
}

impl PlayerData {
    pub fn new(id: PlayerId, name: String) -> Self {
        Self { id, name, order: 0 }
    }
    pub fn ordered(mut self, order: usize) -> Self {
        self.order = order;
        self
    }
}

impl JoinMessage {
    pub fn apply(&self, network: &mut NetworkState, state: &State) {
        require_server!(network);
        require_unapplied!(self);

        debug!("applying join message");

        if let Some(conn) = network.stop_waiting(self.value().code) {
            let player = network.players.insert(
                self.sender(),
                conn,
                self.value().name.clone(),
            );
            network.send_update_event(state);
        }
        self.set_applied();
    }
}

impl ChangeMessage {
    pub fn apply(&self, network: &NetworkState, map: &mut Tilemap, state: &mut State) {
        // require_other!(network,self.sender()); // cannot apply to self
        require_registered!(self);
        require_unapplied!(self);

        debug!("applying change message");

        state.units.execute(map,&self.value().changes);
        self.set_applied();
    }
}

impl ConfirmMessage {
    pub fn apply(&self, network: &mut NetworkState, gui: &mut GuiState) {
        require_registered!(self);
        require_unapplied!(self);

        debug!("applying confirm message");

        network.send_join_event(self.value().code);
        gui.add_message(
            "Server".into(),
            self.value().motd.clone(),
        );
        self.set_applied();
    }
}

impl CreateMessage {
    pub fn apply(&self, network: &NetworkState, map: &mut Tilemap, state: &mut State) {
        require_other!(network,self.sender()); // cannot apply to self
        require_registered!(self);
        require_unapplied!(self);

        debug!("applying create message");

        let data = self.value();
        let mut unit = data.unit.clone().rebuild(&state);
        let point = unit.position().clone();
        unit.insert(map);
        state.units.add(point,unit);

        self.set_applied();

    }
}

impl MoveMessage {
    pub fn apply(&self, network: &NetworkState, map: &mut Tilemap, state: &mut State) {
        require_other!(network,self.sender()); // cannot apply to self
        require_registered!(self);
        require_unapplied!(self);

        debug!("applying move message");

        let data = self.value();
        for (point,moves) in data.moves.iter().group_by(|m| m.1).into_iter() {
            let movement = moves
                .into_iter()
                .map(|m| {
                    warn!("moving {} to {:?}",m.0,point);
                    (m.0,m.2)
                })
                .collect();
            state.units.transfer(map,movement,point);
        }

        self.set_applied();
    }
}

impl ChatMessage {
    pub fn apply(&self, network: &NetworkState, gui: &mut GuiState) {
        require_registered!(self);
        require_unapplied!(self);

        debug!("applying chat message");

        gui.add_message(
            self.name(),
            self.value().message.clone());

        self.set_applied();
    }
}

impl UpdateMessage {
    pub fn apply(&self, network: &mut NetworkState, state: &mut State) {
        require_registered!(self);
        require_unapplied!(self);

        debug!("applying update message");

        let data = self.value();

        network.set_players(data.players.clone());
        state.sync(data);

        self.set_applied();
    }
}