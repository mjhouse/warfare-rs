use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_spicy_networking::{ClientMessage, NetworkMessage, ServerMessage};
use bevy_spicy_networking::{AppNetworkClientMessage,AppNetworkServerMessage};

use crate::generation::Factors;
use crate::generation::{Unit,Place,id::Id};
use crate::objects::Point;
use crate::systems::network::Sync;

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

macro_rules! impl_message {
    ( $n: ident ) => {
        impl_message!($n,u8);
    };
    ( $n: ident, $v: ty ) => {
        #[derive(Serialize, Deserialize, Clone, Debug, Default)]
        pub struct $n {
            pub value: $v
        }

        impl $n {
            pub fn new(value: $v) -> Self {
                Self { value }
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

impl_message!(JoinMessage);

impl_message!(UnitMessage,Unit);

impl_message!(MoveMessage,Place);

impl_message!(ChatMessage,String);

impl_message!(SyncMessage,Sync);

impl_message!(RefreshMessage);

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        register!(app,JoinMessage);
        register!(app,UnitMessage);
        register!(app,MoveMessage);
        register!(app,ChatMessage);
        register!(app,SyncMessage);
        register!(app,RefreshMessage);
    }
}