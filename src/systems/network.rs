use bevy::prelude::*;
use bevy_spicy_networking::{ConnectionId, NetworkData, NetworkServer, NetworkSettings, NetworkClient, ServerNetworkEvent};
use std::net::SocketAddr;
use crate::state::{traits::*, State};
use crate::networking::messages::*;

use std::collections::HashMap;

pub struct NetworkPlugin;
pub struct Player(ConnectionId);

const HOST: usize = 0;
const CONN: usize = 1;
const DCON: usize = 2;
const SEND: usize = 3;

#[derive(Clone,Eq,PartialEq)]
enum Mode {
    None,
    Server,
    Client,
}

pub struct NetworkState {
    flags: [bool;4],
    mode: Mode,
    address: String,
    port: u16,
    message: String,
    pub players: HashMap<ConnectionId,Entity>,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            flags: [false;4],
            mode: Mode::None,
            address: String::new(),
            port: 0,
            message: String::new(),
            players: HashMap::new()
        }
    }
}

impl NetworkState {

    pub fn host(&mut self, address: String, port: u16) {
        self.flags[HOST] = true;
        self.mode = Mode::Server;
        self.address = address;
        self.port = port;
    }
    
    pub fn connect(&mut self, address: String, port: u16) {
        self.flags[CONN] = true;
        self.mode = Mode::Client;
        self.address = address;
        self.port = port;
    }

    pub fn disconnect(&mut self) {
        self.flags[DCON] = true; 
    }

    pub fn send(&mut self, message: String) {
        self.flags[SEND] = true;
        self.message = message;
    }

    pub fn host_requested(&self) -> bool {
        self.flags[HOST] 
    }
    
    pub fn connect_requested(&self) -> bool {
        self.flags[CONN] 
    }
    
    pub fn disconnect_requested(&self) -> bool {
        self.flags[DCON] 
    }

    pub fn send_requested(&self) -> bool {
        self.flags[SEND] 
    }

    pub fn mode(&self) -> Mode {
        self.mode.clone()
    }

    pub fn is_server(&self) -> bool {
        self.mode == Mode::Server
    }

    pub fn is_client(&self) -> bool {
        self.mode == Mode::Client
    }

    pub fn set_server(&mut self) {
        self.mode = Mode::Server;
    }

    pub fn set_client(&mut self) {
        self.mode = Mode::Client;
    }

    pub fn clear_mode(&mut self) {
        self.mode = Mode::None;
    }

    pub fn clear_flags(&mut self) {
        self.flags
            .iter_mut()
            .for_each(|m| *m = false); 
    }

    pub fn address(&self) -> Option<SocketAddr> {
        self.address
            .parse()
            .map(|ip| SocketAddr::new(ip, self.port))
            .ok()
    }
}

fn host_system(
    state: ResMut<State>,
    mut server: ResMut<NetworkServer>,
    mut network: ResMut<NetworkState>,
) {
    if !state.loaded {
        return;
    }

    if !network.host_requested() {
        return;
    }

    // stop listening to current address
    server.stop();
    network.clear_mode();

    // start listening to new address
    match network.address() {
        Some(address) => match server.listen(address) {
            Ok(()) => {
                info!("listening on {:?}",address);
                network.set_server();
            },
            Err(e) => error!("listening failed: {}",e),
        },
        None => error!("bad address"),
    }

    network.clear_flags();
}

fn connect_system(
    state: ResMut<State>,
    mut server: ResMut<NetworkServer>,
    mut client: ResMut<NetworkClient>,
    mut network: ResMut<NetworkState>,
) {
    if !state.loaded {
        return;
    }

    if !network.connect_requested() {
        return;
    }

    // stop listening (if we were)
    server.stop();
    network.clear_mode();

    match network.address() {
        Some(address) => {
            network.set_client();
            client.connect(address, NetworkSettings {
                max_packet_length: 10 * 1024 * 1024,
            });
        },
        None => error!("bad address"),
    }

    network.clear_flags();
}

fn disconnect_system(
    state: ResMut<State>,
    mut server: ResMut<NetworkServer>,
    mut client: ResMut<NetworkClient>,
    mut network: ResMut<NetworkState>,
) {
    if !state.loaded {
        return;
    }

    if !network.disconnect_requested() {
        return;
    }

    server.stop();
    client.disconnect();

    network.clear_mode();
    network.clear_flags();

    info!("disconnected");
}

fn event_system(
    mut commands: Commands,
    server: Res<NetworkServer>,
    mut network: ResMut<NetworkState>,
    mut events: EventReader<ServerNetworkEvent>,
) {

    if !network.is_server() {
        return;
    }

    for event in events.iter() {
        match event {
            ServerNetworkEvent::Connected(id) => {
                info!("player connected; {}", id);
                let entity_id = commands.spawn().insert(Player(*id)).id();
                network.players.insert(*id,entity_id);

                server.broadcast(JoinMessage::default());
            },
            ServerNetworkEvent::Disconnected(id) => {
                info!("player disconnected; {}", id);
                if let Some(entity_id) = network.players.remove(id)
                {
                    commands.entity(entity_id).despawn()
                }
            },
            _ => ()
        };
    }
}

fn server_receive_system(
    state: ResMut<State>,
    server: Res<NetworkServer>,
    network: Res<NetworkState>,
    mut messages: EventReader<NetworkData<ChatMessage>>,
    mut joins: EventReader<NetworkData<JoinMessage>>,
) {
    if !state.loaded {
        return;
    }

    if !network.is_server() {
        return;
    }

    for message in messages.iter() {
        info!("message: {}",message.message);
        server.broadcast(ChatMessage {
            message: message.message.clone(),
        });
    }

    for join in joins.iter() {
        info!("player joined the game");
    }
}

fn client_send_system(
    state: ResMut<State>,
    client: Res<NetworkClient>,
    server: Res<NetworkServer>,
    mut network: ResMut<NetworkState>,
) {
    if !state.loaded {
        return;
    }

    if !network.send_requested() {
        return;
    }

    network.clear_flags();

    if network.message.is_empty() {
        warn!("message is empty");
        return;
    }

    if !network.is_client() {
        return;
    }

    client.send_message(ChatMessage {
        message: network.message.clone(),
    });

    network.message.clear();
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(host_system.system())
           .add_system(connect_system.system())
           .add_system(disconnect_system.system())
           .add_system(event_system.system())
           .add_system(server_receive_system.system())
           .add_system(client_send_system.system());
    }
}