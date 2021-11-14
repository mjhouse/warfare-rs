use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use crate::area::{Biome,Soil};
use crate::state::State;

pub struct GuiPlugin;

#[derive(Default,Debug,Clone)]
pub struct Gui {

}

// called once at startup to load assets
fn gui_resources_system(
    _context: ResMut<EguiContext>, 
    _assets: Res<AssetServer>
) {
    // // no assets yet
    // let handle = assets.load("icon.png");
    // context.set_egui_texture(BEVY_TEXTURE_ID, handle);
}

// called once at startup to init
fn gui_initialize_system(
    _context: ResMut<EguiContext>
) {
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
    mut state: ResMut<State>,
    context: ResMut<EguiContext>,
    _assets: Res<AssetServer>,
) {
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(context.ctx(), |ui| {
            ui.separator();
            ui.heading("Variables");

            ui.horizontal(|ui| {
                ui.label("Seed: ");
                ui.text_edit_singleline(&mut state.terrain.seed);
            });

            ui.horizontal(|ui| {
                ui.label("Size: ");
                ui.text_edit_singleline(&mut state.terrain.size);
            });

            ui.add(egui::Slider::new(&mut state.factors.elevation, 0..=100).text("Elevation"));
            ui.add(egui::Slider::new(&mut state.factors.temperature, 0..=100).text("Temperature"));
            ui.add(egui::Slider::new(&mut state.factors.moisture, 0..=100).text("Moisture"));
            ui.add(egui::Slider::new(&mut state.factors.rockiness, 0..=100).text("Rockiness"));
            ui.add(egui::Slider::new(&mut state.factors.fertility, 0..=100).text("Fertility"));  

            ui.group(|ui|{
                ui.set_width(ui.available_width());
                ui.heading("Biome");
                ui.radio_value(&mut state.factors.biome, Biome::None, "None");
                ui.radio_value(&mut state.factors.biome, Biome::Grassland, "Grassland");
                ui.radio_value(&mut state.factors.biome, Biome::Forest, "Forest");
                ui.radio_value(&mut state.factors.biome, Biome::Desert, "Desert");
                ui.radio_value(&mut state.factors.biome, Biome::Tundra, "Tundra");
                ui.radio_value(&mut state.factors.biome, Biome::Aquatic, "Aquatic");
            });

            
            ui.group(|ui|{
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

            if ui.button("Update").clicked() {
                state.terrain.update = true;
            }

            ui.separator();
            ui.heading("Selection");
            let area = &state.terrain.selected;

            ui.monospace(format!("Id:          {}",area.id()));
            ui.monospace(format!("Location:    {:?}",area.location()));
            ui.monospace(format!("Texture:     {}",area.texture()));
            ui.monospace(format!("Biome:       {}",area.biome()));
            ui.monospace(format!("Soil:        {}",area.soil()));
            ui.monospace(format!("Elevation:   {}",area.elevation()));
            ui.monospace(format!("Temperature: {}",area.temperature()));
            ui.monospace(format!("Fertility:   {}",area.fertility()));
            ui.monospace(format!("Rocks:       {}",area.rocks()));
            ui.monospace(format!("Moisture:    {}",area.moisture()));

            ui.separator();
            ui.heading("Overlay");

            ui.monospace(format!("Current:     {}",state.terrain.overlay));
            ui.monospace("None:        Esc | 0");
            ui.monospace("Biome:       1");
            ui.monospace("Soil:        2");
            ui.monospace("Elevation:   3");
            ui.monospace("Temperature: 4");
            ui.monospace("Fertility:   5");
            ui.monospace("Rocks:       6");
            ui.monospace("Water:       7");

        });
}

impl Plugin for GuiPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(EguiPlugin)
            .add_startup_system(gui_resources_system.system())
            .add_startup_system(gui_initialize_system.system())
            .add_system(gui_configure_system.system())
            .add_system(gui_display_system.system());
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert(Gui::default());
}