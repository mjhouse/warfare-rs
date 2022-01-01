use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::ops::{Deref,DerefMut};
use itertools::Itertools;
use bimap::hash::BiHashMap;

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

use crate::state::{traits::*, State};
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
    flags: [bool;COUNT],
    mode: Mode,
    address: String,
    port: u16,
    motd: String,
    events: NetworkEvents,
    players: BiHashMap<ConnectionId,PlayerId>,
    player:  PlayerId,
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
            flags: [false;COUNT],
            mode: Mode::None,
            address: String::new(),
            port: 0,
            motd: "Welcome to Warfare!".into(),
            events: NetworkEvents::default(),
            players: BiHashMap::new(),
            player: PlayerId::new(),
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

    pub fn create_event(&mut self, sender: PlayerId, unit: Unit) {
        self.messages.push(
            MessageData::Create(
                UnitData {
                    header: HeaderData::new(sender),
                    unit,
                }
            )
        );
    }

    pub fn move_event(&mut self, sender: PlayerId, selections: &Vec<Selection>) {
        self.messages.push(
            MessageData::Move(
                MoveData {
                    header: HeaderData::new(sender),
                    moves: selections
                        .iter()
                        .map(|s| (s.unit(),s.end_point(),s.cost()))
                        .collect(),
                }
            )
        );
    }

    pub fn chat_event(&mut self, sender: PlayerId, message: String) {
        self.messages.push(
            MessageData::Chat(
                ChatData {
                    header: HeaderData::new(sender),
                    message,
                }
            )
        );
    }

    pub fn update_event(&mut self, sender: PlayerId, state: &State) {
        self.messages.push(
            MessageData::Update(
                TerrainData {
                    header: HeaderData::new(sender),
                    seed: state.seed(),
                    turn: state.turn(),
                    factors: state.factors(),
                }
            )
        );
    }

    pub fn join_event(&mut self, sender: PlayerId, player: PlayerId) {
        self.messages.push(
            MessageData::Join(
                JoinData {
                    header: HeaderData::new(sender),
                    player,
                }
            )
        );
    }

    pub fn confirm_event(&mut self, sender: PlayerId, player: PlayerId, motd: String) {
        self.messages.push(
            MessageData::Confirm(
                ConfirmData {
                    header: HeaderData::new(sender).with_target(player),
                    player: player,
                    motd: motd,
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

    pub fn send_move_event(&mut self, selections: &Vec<Selection>) {
        self.flags[SEND] = true;
        self.events.move_event(self.id(),selections);
    }

    pub fn send_chat_event(&mut self, message: String) {
        self.flags[SEND] = true;
        self.events.chat_event(self.id(),message);
    }

    pub fn send_update_event(&mut self, state: &State) {
        self.flags[SEND] = true;
        self.events.update_event(self.id(),state);
    }

    pub fn send_join_event(&mut self, id: PlayerId) {
        self.flags[SEND] = true;
        self.events.join_event(self.id(),id);
    }

    pub fn send_confirm_event(&mut self, id: PlayerId) {
        self.flags[SEND] = true;
        self.events.confirm_event(self.id(),id,self.motd());
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

    pub fn id(&self) -> PlayerId {
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

    pub fn set_id(&mut self, id: PlayerId) {
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
            match message.target().as_ref().map(|t| self.players.get_by_right(t)).flatten() {
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
            T: ServerMessage + NetworkMessage + Clone
    {
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
    if !state.loaded {
        return;
    }

    if !network.host_requested() {
        return;
    }

    // stop listening (if we were)
    server.stop();
    network.clear_mode();
    network.clear_flags();

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
    if !state.loaded {
        return;
    }

    if !network.connect_requested() {
        return;
    }

    // stop listening (if we were)
    server.stop();
    client.disconnect();
    network.clear_mode();
    network.clear_flags();

    match network.address() {
        Some(address) => {
            network.set_client();
            client.connect(address, NetworkSettings {
                max_packet_length: 10 * 1024 * 1024,
            });
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
            ServerNetworkEvent::Connected(connection) => {
                let id = PlayerId::new();
                info!("Player {} joined", id);
                network.players.insert(*connection,id);
                network.send_confirm_event(id);
                network.send_join_event(id);
            },
            ServerNetworkEvent::Disconnected(connection) => {
                if let Some((_,id)) = network.players.remove_by_left(connection)
                {
                    info!("Player {} left", id);
                }
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
    mut create_messages: EventReader<NetworkData<CreateMessage>>,
    mut move_messages: EventReader<NetworkData<MoveMessage>>,
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
    mut refresh_messages: EventReader<NetworkData<RefreshMessage>>,
) {
    if !state.loaded {
        return;
    }

    if !network.is_server() {
        return;
    }

    /*
        join:    ignore
        confirm: ignore
        create:  broadcast
        move:    broadcast
        chat:    broadcast
        update:  ignore
        refresh: broadcast update
    */

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

    mut join_messages: EventReader<NetworkData<JoinMessage>>,
    mut confirm_messages: EventReader<NetworkData<ConfirmMessage>>,
    mut create_messages: EventReader<NetworkData<CreateMessage>>,
    mut move_messages: EventReader<NetworkData<MoveMessage>>,
    mut chat_messages: EventReader<NetworkData<ChatMessage>>,
    mut update_messages: EventReader<NetworkData<UpdateMessage>>,
) {
    if !state.loaded {
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

    for message in join_messages.iter() {
        message.apply(&mut gui);
    }

    for message in confirm_messages.iter() {
        message.apply(&mut network, &mut gui);
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
            MessageData::Chat(v)    => network.send_server_message(&server,ChatMessage::new(v,)),
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