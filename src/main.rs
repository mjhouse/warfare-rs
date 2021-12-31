#![allow(clippy::all)]
// #![allow(warnings)]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::WindowMode;

use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use std::time::Duration;
use log::{Level, SetLoggerError};

mod error;
mod math;

mod behavior;
mod generation;
mod objects;
mod resources;
mod state;
mod systems;
mod networking;

use crate::state::{Action, State};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            filter: "wgpu_core=error".into(),
        })
        .insert_resource(WindowDescriptor {
            title: "Warfare".to_string(),
            width: 800.,
            height: 700.,
            vsync: true,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<State>()
        .init_resource::<systems::gui::GuiState>()
        .init_resource::<systems::network::NetworkState>()

        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)

        .add_plugin(bevy_spicy_networking::ClientPlugin)
        .add_plugin(bevy_spicy_networking::ServerPlugin)
        .add_plugin(networking::messages::MessagePlugin)
        
        // set up camera plugin/system
        .add_plugin(systems::camera::CameraPlugin)
        .add_startup_system(systems::camera::setup.system())

        // set up selection plugin/system
        .add_plugin(systems::selection::SelectionPlugin)
        .add_startup_system(systems::selection::setup.system())

        .add_plugin(systems::gui::GuiPlugin)
        .add_plugin(systems::network::NetworkPlugin)
        .add_plugin(systems::overlay::OverlayPlugin)
        .add_plugin(systems::icon::IconPlugin)
        .add_plugin(systems::generate::GeneratorPlugin)
        .add_plugin(systems::control::ControlPlugin)
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
