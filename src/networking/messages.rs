use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
    };
}

pub struct MessagePlugin;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmptyData;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ConfirmData {
    pub player: Id,
    pub motd: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UnitData {
    pub player: Id,
    pub unit: Unit,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MoveData {
    pub player: Id,
    pub unit: Id,
    pub point: Point,
    pub actions: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ChatData {
    pub player: Id,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TerrainData {
    pub seed: u32,
    pub turn: u32,
    pub factors: Factors
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageData {
    Join(EmptyData),     // player joined
    Confirm(ConfirmData),// confirm connection
    Create(UnitData),    // unit created
    Move(MoveData),      // unit moved
    Chat(ChatData),      // chat message
    Update(TerrainData), // update response
    Refresh(EmptyData),  // request update
}

message!(JoinMessage(EmptyData));
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












// macro_rules! register {
//     ( $a: ident, $n: ty ) => {
//         $a.listen_for_client_message::<$n>();
//         $a.listen_for_server_message::<$n>();
//     }
// }

// macro_rules! impl_message {
//     ( $n: ident ) => {
//         impl_message!($n,u8);
//     };
//     ( $n: ident, $v: ty ) => {
//         #[derive(Serialize, Deserialize, Clone, Debug, Default)]
//         pub struct $n {
//             pub value: $v
//         }

//         impl $n {
//             pub fn new(value: $v) -> Self {
//                 Self { value }
//             }
//         }

//         #[typetag::serde]
//         impl NetworkMessage for $n {}

//         impl ServerMessage for $n {
//             const NAME: &'static str = name!($n);
//         }
        
//         impl ClientMessage for $n {
//             const NAME: &'static str = name!($n);
//         }
//     };
// }

// #[derive(Serialize, Deserialize, Clone, Debug, Default)]
// pub struct Sync {
//     pub seed: u32,
//     pub turn: u32,
//     pub factors: Factors
// }

// #[derive(Clone)]
// pub enum MessageData {
//     Empty,
//     Chat(String),
//     Created(Unit),
//     Moved(Place),
//     Sync(Sync),
// }

// impl_message!(JoinMessage);

// impl_message!(ConfirmMessage,Id);

// impl_message!(UnitMessage,Unit);

// impl_message!(MoveMessage,Place);

// impl_message!(ChatMessage,String);

// impl_message!(SyncMessage,Sync);

// impl_message!(RefreshMessage);

// impl Plugin for MessagePlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         register!(app,JoinMessage);
//         register!(app,ConfirmMessage);
//         register!(app,UnitMessage);
//         register!(app,MoveMessage);
//         register!(app,ChatMessage);
//         register!(app,SyncMessage);
//         register!(app,RefreshMessage);
//     }
// }