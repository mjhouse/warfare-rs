use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::ops::{Deref,DerefMut};
use itertools::Itertools;
use bimap::hash::BiHashMap;
use std::collections::HashMap;
use rand::Rng;

use bevy::prelude::*;
use bevy_tilemap::Tilemap;
use bevy_spicy_networking::{
    ClientMessage,
    NetworkMessage,
    ServerMessage,
    ConnectionId,
    NetworkData,
    NetworkServer,
    NetworkSettings,
    NetworkClient,
    ServerNetworkEvent,
};

use crate::state::{traits::*, State, Flags};
use crate::networking::messages::{self,*};
use crate::generation::{Unit,id::*};
use crate::generation::Factors;
use crate::resources::Label;
use crate::objects::Selection;
use crate::systems::gui::GuiState;

pub struct NetworkPlugin;

const COUNT: usize = 4;

const HOST: usize = 0;
const CONN: usize = 1;
const DCON: usize = 2;
const SEND: usize = 3;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum NetworkFlag {
    Host,
    Hosting,
    Connect,
    Disconnect,
    Send,
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

#[derive(Clone)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
}

pub struct Players {
    ids: BiHashMap<ConnectionId,PlayerId>,
    data: HashMap<PlayerId,Player>,
}

pub struct NetworkState {
    flags: Flags<NetworkFlag>,
    mode: Mode,
    address: String,
    port: u16,
    motd: String,
    events: NetworkEvents,
    pub players: Players,
    player:  PlayerId,
    confirm: bool,
    name: String,
    expecting: HashMap<usize,ConnectionId>,
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

impl Default for Players {
    fn default() -> Self {
        Self {
            ids: BiHashMap::new(),
            data: HashMap::new(),
        }
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
            flags: Flags::new(),
            mode: Mode::None,
            address: String::new(),
            port: 0,
            motd: "Welcome to Warfare!".into(),
            events: NetworkEvents::default(),
            players: Players::default(),
            player: PlayerId::new(),
            confirm: false,
            name: "NAME".into(),
            expecting: HashMap::new(),
        }
    }
}

impl Players {

    pub fn insert(&mut self, id: PlayerId, conn: ConnectionId, name: String) -> Player {
        let player = Player { id, name };
        self.ids.insert(conn,id);
        self.data.insert(id,player.clone());
        player
    }

    pub fn destroy(&mut self, conn: &ConnectionId) -> Option<Player> {
        match self.ids.remove_by_left(conn) {
            Some((_,id)) => {
                self.data.remove(&id)
            },
            None => None,
        }
    }

    pub fn clear(&mut self) {
        self.ids.clear();
        self.data.clear();
    }

    pub fn get(&self, id: &PlayerId) -> Option<&Player> {
        self.data.get(id)
    }

    pub fn get_mut(&mut self, id: &PlayerId) -> Option<&mut Player> {
        self.data.get_mut(id)
    }

    pub fn id(&self, conn: &ConnectionId) -> Option<&PlayerId> {
        self.ids.get_by_left(conn)
    }

    pub fn connection(&self, id: &PlayerId) -> Option<&ConnectionId> {
        self.ids.get_by_right(id)
    }

    pub fn set_name(&mut self, id: &PlayerId, name: String) {
        if let Some(player) = self.get_mut(id) {
            player.name = name;
        }
    }
}

impl NetworkEvents {

    pub fn take(&mut self) -> Vec<MessageData> {
        self.messages.drain(..).collect()
    }

    pub fn add(&mut self, event: MessageData) {
        self.messages.push(event);
    }

    pub fn create_event(&mut self, sender: PlayerId, name: String, unit: Unit) {
        self.messages.push(
            MessageData::Create(
                UnitData {
                    header: HeaderData::new(sender,name),
                    unit,
                }
            )
        );
    }

    pub fn move_event(&mut self, sender: PlayerId, name: String, selections: &Vec<Selection>) {
        self.messages.push(
            MessageData::Move(
                MoveData {
                    header: HeaderData::new(sender,name),
                    moves: selections
                        .iter()
                        .map(|s| (s.unit(),s.end_point(),s.current()))
                        .collect(),
                }
            )
        );
    }

    pub fn chat_event(&mut self, sender: PlayerId, name: String, message: String) {
        self.messages.push(
            MessageData::Chat(
                ChatData {
                    header: HeaderData::new(sender,name),
                    message,
                }
            )
        );
    }

    pub fn update_event(&mut self, sender: PlayerId, name: String, state: &State) {
        self.messages.push(
            MessageData::Update(
                TerrainData {
                    header: HeaderData::new(sender,name),
                    seed: state.seed(),
                    turn: state.turn(),
                    factors: state.factors(),
                }
            )
        );
    }

    pub fn join_event(&mut self, sender: PlayerId, name: String, code: usize) {
        self.messages.push(
            MessageData::Join(
                JoinData {
                    header: HeaderData::new(sender,name.clone()),
                    name,
                    code,
                }
            )
        );
    }

    pub fn confirm_event(&mut self, sender: PlayerId, name: String, motd: String, code: usize) {
        self.messages.push(
            MessageData::Confirm(
                ConfirmData {
                    header: HeaderData::new(sender,name),
                    motd,
                    code,
                }
            )
        );
    }

}

impl NetworkState {

    pub fn host(&mut self, address: String, port: u16) {
        self.flags.set(NetworkFlag::Host);
        self.mode = Mode::Server;
        self.address = address;
        self.port = port;
    }
    
    pub fn connect(&mut self, address: String, port: u16) {
        self.flags.set(NetworkFlag::Connect);
        self.mode = Mode::Client;
        self.address = address;
        self.port = port;
    }

    pub fn disconnect(&mut self) {
        self.flags.set(NetworkFlag::Disconnect);
    }

    pub fn send_create_event(&mut self, unit: Unit) {
        self.flags.set(NetworkFlag::Send);
        self.events.create_event(self.id(), self.name(), unit);
    }

    pub fn send_move_event(&mut self, selections: &Vec<Selection>) {
        self.flags.set(NetworkFlag::Send);
        self.events.move_event(self.id(), self.name(), selections);
    }

    pub fn send_chat_event(&mut self, message: String) {
        self.flags.set(NetworkFlag::Send);
        self.events.chat_event(self.id(), self.name(), message);
    }

    pub fn send_update_event(&mut self, state: &State) {
        self.flags.set(NetworkFlag::Send);
        self.events.update_event(self.id(), self.name(), state);
    }

    pub fn send_join_event(&mut self, code: usize) {
        self.flags.set(NetworkFlag::Send);
        self.events.join_event(self.id(), self.name(), code);
    }

    pub fn send_confirm_event(&mut self, code: usize) {
        self.flags.set(NetworkFlag::Send);
        self.events.confirm_event(self.id(), self.name(), self.motd(),code);
    }

    pub fn host_requested(&self) -> bool {
        self.flags.get(NetworkFlag::Host)
    }
    
    pub fn connect_requested(&self) -> bool {
        self.flags.get(NetworkFlag::Connect)
    }
    
    pub fn disconnect_requested(&self) -> bool {
        self.flags.get(NetworkFlag::Disconnect)
    }

    pub fn send_requested(&self) -> bool {
        self.flags.get(NetworkFlag::Send)
    }

    pub fn id(&self) -> PlayerId {
        self.player.clone()
    }

    pub fn mode(&self) -> Mode {
        self.mode.clone()
    }

    pub fn motd(&self) -> String {
        self.motd.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn is_server(&self) -> bool {
        self.mode == Mode::Server
    }

    pub fn is_client(&self) -> bool {
        self.mode == Mode::Client
    }

    pub fn is_confirmed(&self) -> bool {
        self.confirm
    }

    pub fn is_hosting(&self) -> bool {
        self.flags.get(NetworkFlag::Hosting)
    }

    pub fn set_server(&mut self) {
        self.mode = Mode::Server;
    }

    pub fn set_client(&mut self) {
        self.mode = Mode::Client;
    }

    pub fn set_confirmed(&mut self) {
        self.confirm = true;
    }

    pub fn set_id(&mut self, id: PlayerId) {
        self.player = id;
    }

    pub fn set_motd(&mut self, motd: String) {
        self.motd = motd;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_flag(&mut self, flag: NetworkFlag) {
        self.flags.set(flag);
    }

    pub fn clear_mode(&mut self) {
        self.mode = Mode::None;
    }

    pub fn clear_flags(&mut self) {
        self.flags.clear();
    }

    pub fn clear_flag(&mut self, flag: NetworkFlag) {
        self.flags.unset(flag);
    }

    pub fn clear_confirm(&mut self) {
        self.confirm = false;
    }

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    pub fn start_waiting(&mut self, conn: ConnectionId) -> usize {
        let code = rand::thread_rng().gen();
        self.expecting.insert(code,conn);
        code
    }

    pub fn stop_waiting(&mut self, code: usize) -> Option<ConnectionId> {
        self.expecting.remove(&code)
    }

    pub fn address(&self) -> Option<SocketAddr> {
        self.address
            .parse()
            .map(|ip| SocketAddr::new(ip, self.port))
            .ok()
    }

    fn send_server_message<T>(&mut self, server: &NetworkServer, mut message: T)
        where 
            T: ClientMessage + NetworkMessage + Clone + Message
    {
        let message_id = match message.id() {
            Some(k) => k,
            None => {
                let k = MessageId::new();
                message.set_id(k.clone());
                k
            }
        };

        if !messages::registered(&message_id) {
            match message.target().as_ref().map(|t| self.players.connection(t)).flatten() {
                Some(id) => match server.send_message(*id,message) {
                    Err(e) => warn!("Send failed: {}",e),
                    _ => (),
                },
                None => server.broadcast(message),
            };
            messages::register(&message_id);
        }
    }

    fn send_client_message<T>(&mut self, client: &NetworkClient, message: T)
        where 
            T: ServerMessage + NetworkMessage + Clone + std::fmt::Debug
    {
        warn!("Sending: \n{:?}",&message);
        if let Err(e) = client.send_message(message) {
            warn!("Send failed: {}",e);
        };
    }
}

fn host_system(
    state: ResMut<State>,
    mut server: ResMut<NetworkServer>,
    mut client: ResMut<NetworkClient>,
    mut network: ResMut<NetworkState>,
) {
    if !state.is_loaded() {
        return;
    }

    if !network.host_requested() {
        return;
    }

    server.stop();
    client.disconnect();

    network.clear_mode();
    network.clear_flags();
    network.clear_players();

    // start listening to new address
    match network.address() {
        Some(address) => match server.listen(address) {
            Ok(()) => {
                info!("Hosting at {:?} ({})",address,network.id());
                network.set_server();
                client.connect(address, NetworkSettings {
                    max_packet_length: 10 * 1024 * 1024,
                });
            },
            Err(e) => error!("Hosting failed: {}",e),
        },
        None => error!("No address"),
    }
}

fn connect_system(
    state: ResMut<State>,
    mut server: ResMut<NetworkServer>,
    mut client: ResMut<NetworkClient>,
    mut network: ResMut<NetworkState>,
) {
    if !state.is_loaded() {
        return;
    }

    if !network.connect_requested() {
        return;
    }

    server.stop();
    client.disconnect();

    network.clear_mode();
    network.clear_flags();
    network.clear_players();
    network.clear_confirm();

    let settings = NetworkSettings {
        max_packet_length: 10 * 1024 * 1024,
    };

    match network.address() {
        Some(address) => {
            network.set_client();
            client.connect(address, settings);
        },
        None => error!("Bad address"),
    }
}

fn disconnect_system(
    state: ResMut<State>,
    mut server: ResMut<NetworkServer>,
    mut client: ResMut<NetworkClient>,
    mut network: ResMut<NetworkState>,
) {
    if !state.is_loaded() {
        return;
    }

    if !network.disconnect_requested() {
        return;
    }

    server.stop();
    client.disconnect();

    network.clear_mode();
    network.clear_flags();
    network.clear_players();
    network.clear_confirm();

    info!("Disconnected");
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
            ServerNetworkEvent::Connected(conn) => {
                let code = network.start_waiting(*conn);
                network.send_confirm_event(code);
            },
            ServerNetworkEvent::Disconnected(conn) => {
                network.players.destroy(conn);
            },
            _ => ()
        };
    }
}

fn server_receive_system(
    server: Res<NetworkServer>,

    state: ResMut<State>,
    gui: ResMut<GuiState>,
    mut network: ResMut<NetworkState>,

    mut join_messages: EventReader<NetworkData<JoinMessage>>,
    mut create_messages: EventReader<NetworkData<CreateMessage>>,
    mut move_messages: EventReader<NetworkData<MoveMessage>>,
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
    mut refresh_messages: EventReader<NetworkData<RefreshMessage>>,
) {
    if !state.is_loaded() {
        return;
    }

    if !network.is_server() {
        return;
    }

    /*
        join:    broadcast
        confirm: ignore
        create:  broadcast
        move:    broadcast
        chat:    broadcast
        update:  ignore
        refresh: broadcast update
    */

    for message in join_messages.iter() {
        message.apply(&mut network);
    }

    for message in create_messages.iter() {
        network.send_server_message(&server,(*message).clone());
    }

    for message in move_messages.iter() {
        network.send_server_message(&server,(*message).clone());
    }

    for message in chat_messages.iter() {
        network.send_server_message(&server,(*message).clone());
    }

    if refresh_messages.iter().count() > 0 {
        network.send_update_event(&state);
    }
}

fn client_receive_system(
    client: Res<NetworkClient>,

    mut state: ResMut<State>,
    mut gui: ResMut<GuiState>,
    mut network: ResMut<NetworkState>,
    mut tilemap: Query<&mut Tilemap>,

    mut confirm_messages: EventReader<NetworkData<ConfirmMessage>>,
    mut create_messages: EventReader<NetworkData<CreateMessage>>,
    mut move_messages: EventReader<NetworkData<MoveMessage>>,
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
    mut update_messages: EventReader<NetworkData<UpdateMessage>>,
) {
    if !state.is_loaded() {
        return;
    }

    if !client.is_connected() {
        return;
    }

    /*
        join:    apply
        confirm: apply
        create:  apply
        move:    apply
        chat:    apply
        update:  apply
        refresh: ignore
    */

    let mut map = tilemap.single_mut().expect("Need tilemap");

    for message in confirm_messages.iter() {
        if !network.is_confirmed() {
            message.apply(&mut network, &mut gui);
            network.set_confirmed();
        }
    }

    for message in create_messages.iter() {
        if message.sender() != network.id() {
            message.apply(&network,&mut map, &mut state);
        }
    }

    for message in move_messages.iter() {
        if message.sender() != network.id() {
            message.apply(&network,&mut map, &mut state);
        }
    }

    for message in chat_messages.iter() {
        message.apply(&network, &mut gui);
    }

    for message in update_messages.iter() {
        if message.sender() != network.id() {
            message.apply(&network, &mut state);
        }
    }
}

fn server_send_system(
    state: ResMut<State>,
    client: Res<NetworkClient>,
    server: Res<NetworkServer>,
    mut gui: ResMut<GuiState>,
    mut network: ResMut<NetworkState>,
) {
    if !state.is_loaded() {
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
            MessageData::Chat(v)    => network.send_server_message(&server,ChatMessage::new(v)),
            MessageData::Create(v)  => network.send_server_message(&server,CreateMessage::new(v)),
            MessageData::Move(v)    => network.send_server_message(&server,MoveMessage::new(v)),
            MessageData::Refresh(v) => network.send_client_message(&client,RefreshMessage::new(v)),
            MessageData::Join(v)    => network.send_server_message(&server,JoinMessage::new(v)),
            MessageData::Update(v)  => network.send_server_message(&server,UpdateMessage::new(v)),
            MessageData::Confirm(v) => network.send_server_message(&server,ConfirmMessage::new(v)),
            _ => (),
        };
    }
}

fn client_send_system(
    state: ResMut<State>,
    client: Res<NetworkClient>,
    server: Res<NetworkServer>,
    mut gui: ResMut<GuiState>,
    mut network: ResMut<NetworkState>,
) {
    if !state.is_loaded() {
        return;
    }

    if !network.send_requested() {
        return;
    }

    if !network.is_client() {
        return;
    }

    network.clear_flags();

    for message in network.events.take().into_iter() {
        match message {
            MessageData::Chat(v)    => network.send_client_message(&client,ChatMessage::new(v)),
            MessageData::Create(v)  => network.send_client_message(&client,CreateMessage::new(v)),
            MessageData::Move(v)    => network.send_client_message(&client,MoveMessage::new(v)),
            MessageData::Refresh(v) => network.send_client_message(&client,RefreshMessage::new(v)),
            _ => (),
        };
    }
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
           .add_system(client_send_system.system());
    }
}