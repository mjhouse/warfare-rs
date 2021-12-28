use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use bevy_spicy_networking::{ConnectionId, NetworkData, NetworkServer, NetworkSettings, NetworkClient, ServerNetworkEvent};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::ops::{Deref,DerefMut};

use crate::state::{traits::*, State};
use crate::networking::messages::*;
use crate::generation::{Unit,Id,id};
use crate::generation::Factors;
use crate::resources::Label;
use crate::objects::Selection;
use crate::systems::gui::GuiState;

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

pub struct NetworkEvents {
    messages: Vec<MessageData>,
}

pub struct NetworkState {
    flags: [bool;5],
    mode: Mode,
    address: String,
    port: u16,
    motd: String,
    events: NetworkEvents,
    players: MultiMap<ConnectionId,Id,Entity>,
    player:  Id,
}

impl Deref for NetworkEvents {
    type Target = Vec<MessageData>;
    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

impl DerefMut for NetworkEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.messages
    }
}

impl Default for NetworkEvents {
    fn default() -> Self {
        Self {
            messages: vec![],
        }
    }
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            flags: [false;5],
            mode: Mode::None,
            address: String::new(),
            port: 0,
            motd: "Welcome to Warfare!".into(),
            events: NetworkEvents::default(),
            players: MultiMap::new(),
            player: id::get(),
        }
    }
}

impl NetworkEvents {

    pub fn take(&mut self) -> Vec<MessageData> {
        self.messages.drain(..).collect()
    }

    pub fn create_event(&mut self, sender: Id, unit: Unit) {
        self.messages.push(
            MessageData::Create(
                UnitData {
                    sender,
                    unit,
                }
            )
        );
    }

    pub fn move_event(&mut self, sender: Id, selection: &Selection) {
        self.messages.push(
            MessageData::Move(
                MoveData {
                    sender,
                    unit: selection.unit(),
                    point: selection.end_point(),
                    actions: selection.cost(),
                }
            )
        );
    }

    pub fn chat_event(&mut self, sender: Id, message: String) {
        self.messages.push(
            MessageData::Chat(
                ChatData {
                    sender,
                    message,
                }
            )
        );
    }

    pub fn update_event(&mut self, sender: Id, state: &State) {
        self.messages.push(
            MessageData::Update(
                TerrainData {
                    sender,
                    seed: state.seed(),
                    turn: state.turn(),
                    factors: state.factors(),
                }
            )
        );
    }

    pub fn join_event(&mut self, sender: Id, player: Id) {
        self.messages.push(
            MessageData::Join(
                JoinData {
                    sender,
                    player,
                }
            )
        );
    }

    pub fn confirm_event(&mut self, sender: Id, player: Id, motd: String) {
        self.messages.push(
            MessageData::Confirm(
                ConfirmData {
                    sender,
                    player,
                    motd,
                }
            )
        );
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

    pub fn send_create_event(&mut self, unit: Unit) {
        self.flags[SEND] = true;
        self.events.create_event(self.id(),unit);
    }

    pub fn send_move_event(&mut self, selection: &Selection) {
        self.flags[SEND] = true;
        self.events.move_event(self.id(),selection);
    }

    pub fn send_chat_event(&mut self, message: String) {
        self.flags[SEND] = true;
        self.events.chat_event(self.id(),message);
    }

    pub fn send_update_event(&mut self, state: &State) {
        self.flags[SEND] = true;
        self.events.update_event(self.id(),state);
    }

    pub fn send_join_event(&mut self, id: Id) {
        self.flags[SEND] = true;
        self.events.join_event(self.id(),id);
    }

    pub fn send_confirm_event(&mut self, id: Id) {
        self.flags[SEND] = true;
        self.events.confirm_event(self.id(),id,self.motd());
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

    pub fn motd(&self) -> String {
        self.motd.clone()
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

    pub fn set_motd(&mut self, motd: String) {
        self.motd = motd;
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

    pub fn connection(&self, id: Id) -> Option<ConnectionId> {
        self.players
            .iter()
            .find(|(_,n)| n.0 == id)
            .map( |(k,_)| k.clone())
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
                network.send_join_event(id);
                network.send_confirm_event(id);

                // info!("sending confirm; {}", id);
                // server.send_message(conn,ConfirmMessage::new(ConfirmData {
                //     sender: network.id(),
                //     player: id,
                //     motd: "Welcome to Warfare!".into(),
                // }));
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
    mut gui: ResMut<GuiState>,
    server: Res<NetworkServer>,
    mut network: ResMut<NetworkState>,
    mut map_query: Query<&mut Tilemap>,
    mut create_messages: EventReader<NetworkData<CreateMessage>>,
    mut move_messages: EventReader<NetworkData<MoveMessage>>,
    mut update_messages: EventReader<NetworkData<UpdateMessage>>,
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
    mut join_messages: EventReader<NetworkData<JoinMessage>>,
    mut confirm_messages: EventReader<NetworkData<ConfirmMessage>>,
) {
    if !state.loaded {
        return;
    }

    if !network.is_server() {
        return;
    }

    let mut tilemap = map_query.single_mut().expect("Need tilemap");

    for message in create_messages.iter() {
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
    if update_messages.iter().count() > 0 {
        info!("responding to sync request");
        network.sync();
    }

    for message in chat_messages.iter() {
        message.apply(&mut gui);
        let id = message.value().sender.clone();
        let msg = message.value().message.clone();
        info!("message: {}",msg);
        server.broadcast(ChatMessage::new(ChatData {
            sender: id,
            message: msg,
        }));
    }

    for message in chat_messages.iter() {
        message.apply(&mut gui);
    }

    for join in join_messages.iter() {
        info!("player joined the game");
    }
}

fn client_receive_system(
    mut state: ResMut<State>,
    mut gui: ResMut<GuiState>,
    server: Res<NetworkServer>,
    mut network: ResMut<NetworkState>,
    mut map_query: Query<&mut Tilemap>,
    mut create_messages: EventReader<NetworkData<CreateMessage>>,
    mut move_messages: EventReader<NetworkData<MoveMessage>>,
    mut update_messages: EventReader<NetworkData<UpdateMessage>>,
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
    mut join_messages: EventReader<NetworkData<JoinMessage>>,
    mut confirm_messages: EventReader<NetworkData<ConfirmMessage>>,
) {
    if !state.loaded {
        return;
    }

    if !network.is_client() {
        return;
    }

    let mut tilemap = map_query.single_mut().expect("Need tilemap");

    for (point,messages) in move_messages.iter().group_by(|m| m.value().point).into_iter() {
            let ids = messages
                .into_iter()
                .map(|m| m.value().unit)
                .collect();
            state.units.select(&ids);
            state.units.move_selection(&mut tilemap,&point);
            state.units.select_none_free();
    }

    for message in create_messages.iter() {
        message.apply(&mut tilemap, &mut state);
    }

    for message in update_messages.iter() {
        message.apply(&mut state);
    }

    for message in chat_messages.iter() {
        message.apply(&mut gui);
    }

    for message in confirm_messages.iter() {
        message.apply(&mut network);
    }
}

fn server_send_system(
    state: ResMut<State>,
    mut gui: ResMut<GuiState>,
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

    for message in network.events.take().into_iter() {
        match message {
            MessageData::Chat(v) => {
                let message = ChatMessage::new(v);
                server.broadcast(message.clone());
                message.apply(&mut gui);
            },
            MessageData::Update(v) => server.broadcast(UpdateMessage::new(v)),
            MessageData::Create(v) => server.broadcast(CreateMessage::new(v)),
            MessageData::Move(v) => server.broadcast(MoveMessage::new(v)),
            MessageData::Confirm(v) => {
                if let Some(c) = network.connection(v.player) {
                    server.send_message(c,ConfirmMessage::new(v));
                }
            },
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

    for message in network.events.drain(..) {
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

    network.send_update_event(&state);
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