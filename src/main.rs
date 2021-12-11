#![allow(clippy::all)]
// #![allow(warnings)]

use bevy::window::WindowMode;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

mod math;
mod error;

mod behavior;
mod resources;
mod generation;
mod objects;
mod systems;
mod state;

use crate::state::{State,Action};

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
        .add_plugin(systems::camera::CameraPlugin)
        .add_startup_system(systems::camera::setup.system())

        // set up selection plugin/system
        .add_plugin(systems::selection::SelectionPlugin)
        .add_startup_system(systems::selection::setup.system())

        .add_plugin(systems::overlay::OverlayPlugin)    // set up overlay plugin
        .add_plugin(systems::icon::IconPlugin)          // set up window icon plugin
        .add_plugin(systems::gui::GuiPlugin)            // set up gui plugin
        .add_plugin(systems::generate::GeneratorPlugin) // set up generator plugin
        .add_plugin(systems::control::ControlPlugin)    // add unit control/placement plugin
        
        .add_startup_system(setup.system())
        .run()
}

fn setup(
    //mut commands: Commands,
    mut state: ResMut<State>,
    assets: Res<AssetServer>,
) {
    state.textures.handles = assets
        .load_folder("textures")
        .expect("Could not load textures");

    state.events.send(Action::UpdateTerrain);
}