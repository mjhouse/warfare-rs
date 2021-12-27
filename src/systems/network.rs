use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use bevy_spicy_networking::{ConnectionId, NetworkData, NetworkServer, NetworkSettings, NetworkClient, ServerNetworkEvent};
use std::net::SocketAddr;
use std::collections::HashMap;

use crate::state::{traits::*, State};
use crate::networking::messages::*;
use crate::generation::{Unit,Id,id};
use crate::generation::Factors;
use crate::resources::Label;
use bevy_tilemap::Tilemap;

use itertools::Itertools;
use multi_map::MultiMap;

pub struct NetworkPlugin;
pub struct Player(ConnectionId);

const HOST: usize = 0;
const CONN: usize = 1;
const DCON: usize = 2;
const SEND: usize = 3;
const SYNC: usize = 4;

pub struct Connection {
    connection_id: ConnectionId,
    entity: Entity,
    id: Id,
}

#[derive(Clone,Eq,PartialEq)]
pub enum Mode {
    None,
    Server,
    Client,
}

pub struct NetworkState {
    flags: [bool;5],
    mode: Mode,
    address: String,
    port: u16,
    messages: Vec<MessageData>,
    players: MultiMap<ConnectionId,Id,Entity>,
    player:  Id,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            flags: [false;5],
            mode: Mode::None,
            address: String::new(),
            port: 0,
            messages: vec![],
            players: MultiMap::new(),
            player: id::get(),
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

    pub fn send(&mut self, message: MessageData) {
        self.flags[SEND] = true;
        self.messages.push(message);
    }

    pub fn sync(&mut self) {
        self.flags[SYNC] = true;
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

    pub fn sync_requested(&self) -> bool {
        self.flags[SYNC] 
    }

    pub fn id(&self) -> Id {
        self.player.clone()
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

    pub fn set_id(&mut self, id: Id) {
        self.player = id;
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
                info!("listening on {:?} (as player {})",address,network.id());
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
            ServerNetworkEvent::Connected(connection) => {
                info!("player connected; {}", connection);
                let id = id::get();
                let conn = connection.clone();
                let entity = commands
                    .spawn()
                    .insert(Player(conn.clone()))
                    .id();
                
                network.players.insert(conn.clone(),id,entity);
                server.broadcast(JoinMessage::default());

                info!("sending confirm; {}", id);
                server.send_message(conn,ConfirmMessage::new(ConfirmData {
                    player: id,
                    motd: "Welcome to Warfare!".into(),
                }));
            },
            ServerNetworkEvent::Disconnected(connection) => {
                info!("player disconnected; {}", connection);
                if let Some(entity) = network.players.remove(connection)
                {
                    commands.entity(entity).despawn()
                }
            },
            _ => ()
        };
    }
}

fn server_receive_system(
    mut state: ResMut<State>,
    server: Res<NetworkServer>,
    mut network: ResMut<NetworkState>,
    mut map_query: Query<&mut Tilemap>,
    mut unit_messages: EventReader<NetworkData<CreateMessage>>,
    mut move_messages: EventReader<NetworkData<MoveMessage>>,
    mut sync_messages: EventReader<NetworkData<UpdateMessage>>,
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
    mut join_messages: EventReader<NetworkData<JoinMessage>>,
) {
    if !state.loaded {
        return;
    }

    if !network.is_server() {
        return;
    }

    let mut tilemap = map_query.single_mut().expect("Need tilemap");

    for message in unit_messages.iter() {
        let mut unit = message.value().unit.clone();
        let point = unit.position().clone();

        *(unit.texture_mut()) = state.textures.get(Label::Unit);

        unit.insert(&mut tilemap);
        state.units.add(point,unit);
    }

    for (point,messages) in move_messages.iter().group_by(|m| m.value().point).into_iter() {
        let ids = messages
            .into_iter()
            .map(|m| m.value().unit)
            .collect();
        state.units.select(&ids);
        state.units.move_selection(&mut tilemap,&point);
        state.units.select_none_free();
    }

    // if the server receives a sync message, it just echos
    // it's state back to the clients.
    if sync_messages.iter().count() > 0 {
        info!("responding to sync request");
        network.sync();
    }

    for message in chat_messages.iter() {
        let id = message.value().player.clone();
        let msg = message.value().message.clone();
        info!("message: {}",msg);
        server.broadcast(ChatMessage::new(ChatData {
            player: id,
            message: msg,
        }));
    }

    for join in join_messages.iter() {
        info!("player joined the game");
    }
}

fn client_receive_system(
    mut state: ResMut<State>,
    server: Res<NetworkServer>,
    mut network: ResMut<NetworkState>,
    mut map_query: Query<&mut Tilemap>,
    mut unit_messages: EventReader<NetworkData<CreateMessage>>,
    mut move_messages: EventReader<NetworkData<MoveMessage>>,
    mut sync_messages: EventReader<NetworkData<UpdateMessage>>,
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
    mut join_messages: EventReader<NetworkData<JoinMessage>>,
    mut conn_messages: EventReader<NetworkData<ConfirmMessage>>,
) {
    if !state.loaded {
        return;
    }

    if !network.is_client() {
        return;
    }

    let mut tilemap = map_query.single_mut().expect("Need tilemap");

    for message in unit_messages.iter() {
        let mut unit = message.value().unit.clone();
        let point = unit.position().clone();

        *(unit.texture_mut()) = state.textures.get(Label::Unit);

        unit.insert(&mut tilemap);
        state.units.add(point,unit);
    }

    for (point,messages) in move_messages.iter().group_by(|m| m.value().point).into_iter() {
            let ids = messages
                .into_iter()
                .map(|m| m.value().unit)
                .collect();
            state.units.select(&ids);
            state.units.move_selection(&mut tilemap,&point);
            state.units.select_none_free();
    }

    for message in unit_messages.iter() {
        dbg!(&message.value().unit);
    }

    for message in sync_messages.iter() {
        info!("synchronizing with server");
        state.sync(message.value().clone());
    }

    for message in chat_messages.iter() {
        info!("message: {}",message.value().message);
    }

    for message in conn_messages.iter() {
        let id = message.value().player.clone();
        let motd = message.value().motd.clone();
        network.set_id(id);
        info!("ID:   {}",id);
        info!("MOTD: {}",motd);
    }
}

fn server_send_system(
    state: ResMut<State>,
    server: Res<NetworkServer>,
    mut network: ResMut<NetworkState>,
) {
    if !state.loaded {
        return;
    }

    if !network.is_server() {
        return;
    }

    if !network.send_requested() {
        return;
    }

    network.clear_flags();

    for message in network.messages.drain(..) {
        match message {
            MessageData::Chat(v) => server.broadcast(ChatMessage::new(v)),
            MessageData::Update(v) => server.broadcast(UpdateMessage::new(v)),
            MessageData::Create(v) => server.broadcast(CreateMessage::new(v)),
            MessageData::Move(v) => server.broadcast(MoveMessage::new(v)),
            _ => (),
        };
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

    if !network.is_client() {
        return;
    }

    network.clear_flags();

    for message in network.messages.drain(..) {
        match message {
            MessageData::Chat(v)   => { client.send_message(ChatMessage::new(v)); },
            MessageData::Create(v) => { client.send_message(CreateMessage::new(v)); },
            MessageData::Move(v)   => { client.send_message(MoveMessage::new(v)); },
            _ => (),
        };
    }
}

fn sync_system(
    state: ResMut<State>,
    client: Res<NetworkClient>,
    server: Res<NetworkServer>,
    mut network: ResMut<NetworkState>,
) {
    if !state.loaded {
        return;
    }

    if !network.sync_requested() {
        return;
    }

    network.clear_flags();

    if !network.is_server() {
        return;
    }

    let sync = MessageData::Update(TerrainData {
        seed: state.seed(),
        turn: state.turn(),
        factors: state.factors(),
    });

    network.send(sync);
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(host_system.system())
           .add_system(connect_system.system())
           .add_system(disconnect_system.system())
           .add_system(event_system.system())
           .add_system(server_receive_system.system())
           .add_system(client_receive_system.system())
           .add_system(server_send_system.system())
           .add_system(client_send_system.system())
           .add_system(sync_system.system());
    }
}