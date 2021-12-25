use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_spicy_networking::{ClientMessage, NetworkMessage, ServerMessage};

use bevy_spicy_networking::{AppNetworkClientMessage,AppNetworkServerMessage};

pub struct MessagePlugin;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct JoinMessage;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub message: String,
}

#[typetag::serde]
impl NetworkMessage for ChatMessage {}

impl ServerMessage for ChatMessage {
    const NAME: &'static str = "warfare:ChatMessage";
}

impl ClientMessage for ChatMessage {
    const NAME: &'static str = "warfare:ChatMessage";
}

#[typetag::serde]
impl NetworkMessage for JoinMessage {}

impl ServerMessage for JoinMessage {
    const NAME: &'static str = "warfare:JoinMessage";
}

impl ClientMessage for JoinMessage {
    const NAME: &'static str = "warfare:JoinMessage";
}

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.listen_for_client_message::<ChatMessage>();
        app.listen_for_server_message::<ChatMessage>();
        app.listen_for_client_message::<JoinMessage>();
        app.listen_for_server_message::<JoinMessage>();
    }
}