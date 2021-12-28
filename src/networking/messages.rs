use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy_tilemap::Tilemap;

use bevy_spicy_networking::{
    ClientMessage,
    NetworkMessage,
    ServerMessage,
    AppNetworkClientMessage,
    AppNetworkServerMessage
};

use crate::generation::Factors;
use crate::generation::{Unit,id::Id};
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
    ( $n: ident ( $v: ty )) => {
        #[derive(Serialize, Deserialize, Clone, Debug, Default)]
        pub struct $n {
            inner: $v,
        }

        impl $n {
            pub fn new(v: $v) -> Self {
                Self { 
                    inner: v,
                }
            }

            pub fn value(&self) -> &$v {
                &self.inner
            }

            pub fn take(self) -> $v {
                self.inner
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
    };
}

pub struct MessagePlugin;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmptyData;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct JoinData {
    pub sender: Id,
    pub player: Id,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ConfirmData {
    pub sender: Id,
    pub player: Id,
    pub motd: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UnitData {
    pub sender: Id,
    pub unit: Unit,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MoveData {
    pub sender: Id,
    pub unit: Id,
    pub point: Point,
    pub actions: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ChatData {
    pub sender: Id,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TerrainData {
    pub sender: Id,
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

message!(JoinMessage(JoinData));
message!(ConfirmMessage(ConfirmData));
message!(CreateMessage(UnitData));
message!(MoveMessage(MoveData));
message!(ChatMessage(ChatData));
message!(UpdateMessage(TerrainData));
message!(RefreshMessage(EmptyData));

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

impl JoinMessage {
    pub fn apply(&self, gui: &mut GuiState) {
        // add a visible join message to chat window
        let data = self.value();
        let message = format!("{} joined the game",data.player);
        gui.add_message(data.sender,message);
    }
}

impl ConfirmMessage {
    pub fn apply(&self, network: &mut NetworkState) {
        // update player id to match the one issued by server
        let data = self.value();
        network.set_id(data.player);
        network.set_motd(data.motd.clone());
    }
}

impl CreateMessage {
    pub fn apply(&self, map: &mut Tilemap, state: &mut State) {
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
    pub fn apply(&self, /* map: &mut Tilemap, state: &mut State */) {
        // update position of unit in tilemap and unit map
    }
}

impl ChatMessage {
    pub fn apply(&self, gui: &mut GuiState) {
        // add visible chat message to chat window
        let data = self.value();
        gui.add_message(data.sender,data.message.clone());
    }
}

impl UpdateMessage {
    pub fn apply(&self, state: &mut State) {
        // update terrain and factors then signal re-generate
        let data = self.value();
        state.sync(data.clone());
    }
}

impl RefreshMessage {
    pub fn apply(&self, /* network: &mut Network */) {
        // enqueue an update message in reply to sender
    }
}