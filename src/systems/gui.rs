use crate::generation::{Biome, Soil};
use crate::state::traits::HasId;
use crate::state::{Action, State, Flags};
use crate::systems::selection::Selection;
use crate::systems::network::NetworkState;
use crate::networking::messages::*;
use crate::generation::{PlayerId,Id};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum GuiFlag {
    NetworkWindow,
    UnitWindow,
    ChatWindow,
    InfoWindow,
}

pub struct Message {
    player:  String,
    content: String,
}

pub struct GuiState {
    flags: Flags<GuiFlag>,
    ip: String,
    port: u16,
    name: String,
    unit_name: String,
    message: String,
    pub history: Vec<Message>,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            flags: Flags::new(),
            ip: "127.0.0.1".into(),
            port: 8080,
            name: "None".into(),
            unit_name: "".into(),
            message: "".into(),
            history: vec![],
        }
    }
}

impl GuiState {
    pub fn add_message(&mut self, player: String, content: String) {
        self.history.push(Message {
            player,
            content,
        });
    }
}

pub struct GuiPlugin;

// called once at startup to load assets
fn gui_resources_system(_context: ResMut<EguiContext>, _assets: Res<AssetServer>) {
    // // no assets yet
    // let handle = assets.load("icon.png");
    // context.set_egui_texture(BEVY_TEXTURE_ID, handle);
}

// called once at startup to init
fn gui_initialize_system(_context: ResMut<EguiContext>) {
    // // https://docs.rs/egui/0.14.2/egui/style/struct.Visuals.html
    // use equi::Color32;

    // context.ctx().set_visuals(egui::Visuals {
    //     // dark_mode: true,
    //     // ..Default::default()
    // });
}

// called repeatedly, react to window changes
fn gui_configure_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Some(window) = windows.get_primary() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.0
            } else {
                1.0 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
}

// called repeatedly to update ui
fn gui_display_system(
    mut gui: ResMut<GuiState>,
    mut network: ResMut<NetworkState>,
    mut state: ResMut<State>,
    windows: Res<Windows>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Selection>,
    context: ResMut<EguiContext>,
    _assets: Res<AssetServer>,
) {
    let window = windows.get_primary().unwrap();
    let mut selection = query.single_mut().expect("Need selection");

    selection.hovering = window.cursor_position().is_some();

    egui::SidePanel::left("side_panel")
    .default_width(200.0)
    .show(context.ctx(), |ui| {
        ui.separator();
        ui.heading("Variables");

        ui.horizontal(|ui| {
            if state.terrain.seed.is_empty() {
                state.terrain.seed = "0".into();
            }

            ui.label("Seed: ");
            ui.text_edit_singleline(&mut state.terrain.seed);
        });

        ui.add(egui::Slider::new(&mut state.factors.elevation, 0..=100).text("Elevation"));
        ui.add(
            egui::Slider::new(&mut state.factors.temperature, 0..=100).text("Temperature"),
        );
        ui.add(egui::Slider::new(&mut state.factors.moisture, 0..=100).text("Moisture"));
        ui.add(egui::Slider::new(&mut state.factors.rockiness, 0..=100).text("Rockiness"));
        ui.add(egui::Slider::new(&mut state.factors.fertility, 0..=100).text("Fertility"));

        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.heading("Biome");
            ui.radio_value(&mut state.factors.biome, Biome::None, "None");
            ui.radio_value(&mut state.factors.biome, Biome::Grassland, "Grassland");
            ui.radio_value(&mut state.factors.biome, Biome::Forest, "Forest");
            ui.radio_value(&mut state.factors.biome, Biome::Desert, "Desert");
            ui.radio_value(&mut state.factors.biome, Biome::Tundra, "Tundra");
            ui.radio_value(&mut state.factors.biome, Biome::Aquatic, "Aquatic");
        });

        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.heading("Soil");
            ui.radio_value(&mut state.factors.soil, Soil::None, "None");
            ui.radio_value(&mut state.factors.soil, Soil::Clay, "Clay");
            ui.radio_value(&mut state.factors.soil, Soil::Sand, "Sand");
            ui.radio_value(&mut state.factors.soil, Soil::Silt, "Silt");
            ui.radio_value(&mut state.factors.soil, Soil::Peat, "Peat");
            ui.radio_value(&mut state.factors.soil, Soil::Chalk, "Chalk");
            ui.radio_value(&mut state.factors.soil, Soil::Loam, "Loam");
        });

        ui.horizontal(|ui| {
            if ui.button("Update").clicked() {
                state.events.send(Action::UpdateTerrain);
            }

            if ui.button("End Turn").clicked() {
                state.end_turn();
                state.events.send(Action::UpdateTerrain);
            }

            if ui.button("Units").clicked() {
                gui.flags.toggle(GuiFlag::UnitWindow);
            }

            if ui.button("Network").clicked() {
                gui.flags.toggle(GuiFlag::NetworkWindow);
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Chat").clicked() {
                gui.flags.toggle(GuiFlag::ChatWindow);
            }
        });

        ui.label(format!("{}", state.calendar));

        ui.separator();
        ui.heading("Selection");
        let area = &state.terrain.selected;

        ui.monospace(format!("Id:          {}", area.id()));
        ui.monospace(format!("Impedence:   {}", area.impedance()));
        ui.monospace(format!("Location:    {:?}", area.location()));
        ui.monospace(format!("Texture:     {}", area.texture().unwrap_or(0)));
        ui.monospace(format!("Biome:       {}", area.biome()));
        ui.monospace(format!("Soil:        {}", area.soil()));
        ui.monospace(format!("Elevation:   {}", area.elevation()));
        ui.monospace(format!("Temperature: {}", area.temperature()));
        ui.monospace(format!("Fertility:   {}", area.fertility()));
        ui.monospace(format!("Rocks:       {}", area.rocks()));
        ui.monospace(format!("Moisture:    {}", area.moisture()));

        ui.separator();
        ui.heading("Overlay");

        ui.monospace(format!("Current:     {}", state.terrain.overlay));
        ui.monospace("None:        Esc | 0");
        ui.monospace("Biome:       1");
        ui.monospace("Soil:        2");
        ui.monospace("Elevation:   3");
        ui.monospace("Temperature: 4");
        ui.monospace("Fertility:   5");
        ui.monospace("Rocks:       6");
        ui.monospace("Water:       7");

        if hovered(window,ui) {
            selection.hovering = false;
        }
    });

    let units = state.find_units(&selection.selected);
    gui.flags.update(GuiFlag::InfoWindow,!units.is_empty());

    if gui.flags.get(GuiFlag::InfoWindow) {
        egui::Window::new("Unit")
        .default_width(300.0)
        .default_height(200.0)
        .collapsible(false)
        .show(context.ctx(), |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());

            for unit in units.into_iter() {
                let mut select = selection.has(unit);

                ui.add_space(10.);
                ui.collapsing(format!("Unit {}",unit.id()), |ui| {
                    ui.add_space(10.);
                    ui.heading("Info");
                    
                    ui.monospace(format!("Name:   {}", unit.name()));
                    ui.monospace(format!("Player: {}", unit.player_name()));
                    ui.monospace(format!("Type:   {:?}", unit.specialty()));
                    ui.monospace(format!("AP:     {}", unit.actions()));
                    ui.monospace(format!("Max AP: {}", unit.max_actions()));
    
                    ui.add_space(10.);
                    ui.checkbox(&mut select,"Select unit");
                    ui.add_space(10.);
                    ui.collapsing("Soldiers", |ui| {
                        egui::ScrollArea::from_max_height(400.)
                            .show(ui, |ui| {
                            ui.add_space(5.);
                            for soldier in unit.soldiers() {
                                let (h, mh) = soldier.health();
                                let (m, mm) = soldier.morale();
                                let (d, md) = soldier.defense();
                                let (a, ma) = soldier.attack();
                                let p = soldier.actions();
                                let mp = soldier.max_actions();
    
                                ui.group(|ui| {
                                    ui.set_width(ui.available_width());
                                    ui.monospace(format!("  Name:    {}", soldier.name()));
                                    ui.monospace(format!("  Age:     {}", soldier.age()));
                                    ui.monospace(format!("  Sex:     {:?}", soldier.sex()));
                                    ui.monospace(format!("  Weight:  {}kg", soldier.weight()));
                                    ui.monospace(format!("  Height:  {}cm", soldier.height()));
                                    ui.monospace(format!("  AP:      {} / {}", p, mp));
                                    ui.monospace(format!("  HP:      {} / {}", h, mh));
                                    ui.monospace(format!("  Morale:  {} / {}", m, mm));
                                    ui.monospace(format!("  Defense: {} / {}", d, md));
                                    ui.monospace(format!("  Attack:  {} / {}", a, ma));
                                });
                                ui.add_space(5.);
                            }
                        });
                    });
                });

                if select {
                    selection.add(unit);
                }
                else {
                    selection.remove(unit);
                }
            }

            if hovered(window,ui) {
                selection.hovering = false;
            }
        });
    }

    if gui.flags.get(GuiFlag::NetworkWindow) {
        egui::Window::new("Network")
        .default_width(300.0)
        .default_height(80.0)
        .collapsible(false)
        .resizable(false)
        .show(context.ctx(), |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());

            ui.horizontal(|ui| {
                ui.monospace("ip:   ");
                ui.text_edit_singleline(&mut gui.ip);
            });

            ui.horizontal(|ui| {
                ui.monospace("port: ");

                let mut port = gui.port.to_string();
                ui.text_edit_singleline(&mut port);
                if let Ok(v) = port.parse::<u16>() {
                    gui.port = v;
                }
            });

            ui.horizontal(|ui| {
                ui.monospace("name: ");
                ui.text_edit_singleline(&mut gui.name);
            });

            ui.horizontal(|ui| {
                if ui.button("Host").clicked() {
                    network.set_name(gui.name.clone());
                    network.host(gui.ip.clone(),gui.port);
                }

                if ui.button("Connect").clicked() {
                    network.set_name(gui.name.clone());
                    network.connect(gui.ip.clone(),gui.port);
                }

                if ui.button("Disconnect").clicked() {
                    network.disconnect();
                }
            });

            if hovered(window,ui) {
                selection.hovering = false;
            }
        });
    }

    if gui.flags.get(GuiFlag::ChatWindow) {
        egui::Window::new("Chat")
        .default_width(300.0)
        .default_height(400.0)
        .show(context.ctx(), |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());

            let text_style = egui::TextStyle::Body;
            let row_height = ui.fonts()[text_style].row_height();
            let num_rows   = gui.history.len();

            egui::ScrollArea::vertical()
                .max_height(400.)
                .stick_to_bottom()
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, gui.history.len(), |ui, row_range| {
                    for row in row_range {
                        if let Some(message) = gui.history.get(row) {
                            let player = &message.player;
                            let content = &message.content;
                            ui.label(format!("{}: {}",player,content));
                        }
                    }
                });

            ui.add_space(10.);
            ui.horizontal(|ui| {
                let response = ui.text_edit_singleline(&mut gui.message);

                if keyboard.just_pressed(KeyCode::Return) && response.lost_focus() {
                    network.send_chat_event(gui.message.clone());
                    gui.message.clear();
                    response.request_focus();
                }

                if ui.button("Send").clicked() {
                    network.send_chat_event(gui.message.clone());
                    gui.message.clear();
                }
            });

            if hovered(window,ui) {
                selection.hovering = false;
            }
        });
    }

    if gui.flags.get(GuiFlag::UnitWindow) {
        egui::Window::new("Units")
            .default_width(300.0)
            .default_height(400.0)
            .show(context.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.monospace("name: ");
                    ui.text_edit_singleline(&mut gui.unit_name);
                });

                ui.horizontal(|ui| {
                    if ui.button("Infantry").clicked() {
                        selection.place_request(gui.unit_name.clone());
                    }

                    if ui.button("Armor").clicked() {
                        selection.place_request(gui.unit_name.clone());
                    }

                    if ui.button("Militia").clicked() {
                        selection.place_request(gui.unit_name.clone());
                    }
                });

                if hovered(window,ui) {
                    selection.hovering = false;
                }
            });
    }
    
    // dbg!(selection.hovering);
}

fn hovered(window: &Window, ui: &egui::Ui) -> bool {
    match window.cursor_position() {
        Some(mut p) => {
            let mut r = ui.min_rect();
            let s = ui.style();
    
            p.y = window.height() - p.y;
    
            let pad  = s.spacing.window_padding;
            let side = s.interaction.resize_grab_radius_side;
    
            r.min.x -= pad.x + side;
            r.max.x += pad.x + side + 5.;
            r.min.y -= pad.y + side + 25.;
            r.max.y += pad.y + side;
    
            p.x >= r.min.x && 
            p.x <= r.max.x && 
            p.y >= r.min.y && 
            p.y <= r.max.y
        }
        None => false
    }
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(EguiPlugin)
            .add_startup_system(gui_resources_system.system())
            .add_startup_system(gui_initialize_system.system())
            .add_system(gui_configure_system.system())
            .add_system(gui_display_system.system());
    }
}
