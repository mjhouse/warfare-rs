#![allow(clippy::all)]
// #![allow(warnings)]

use bevy::window::WindowMode;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

mod math;
mod camera;
mod area;
mod generate;
mod generator;
mod overlay;
mod gui;
mod selection;
mod state;
mod spectrum;
mod error;

use state::State;

fn main() {
    pretty_env_logger::init();
    
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(WindowDescriptor {
            title: "Warfare".to_string(),
            width: 1036.,
            height: 1036.,
            vsync: true,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<State>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)

        // set up camera plugin/system
        .add_plugin(camera::CameraPlugin)
        .add_startup_system(camera::setup.system())

        // set up selection plugin/system
        .add_plugin(selection::SelectionPlugin)
        .add_startup_system(selection::setup.system())

        // set up overlay plugin/system
        .add_plugin(overlay::OverlayPlugin)
        .add_startup_system(overlay::setup.system())
        
        // set up gui plugin/system
        .add_plugin(gui::GuiPlugin)
        .add_startup_system(gui::setup.system())

        // set up generator plugin/system
        .add_plugin(generate::GeneratorPlugin)
        .add_startup_system(generate::setup.system())

        .add_startup_system(setup.system())
        .run()
}

fn setup(
    mut _commands: Commands,
    mut state: ResMut<State>,
    asset_server: Res<AssetServer>,
) {
    state.resources.textures = asset_server.load_folder("textures").unwrap();
    state.terrain.update = true; // force terrain generation
}