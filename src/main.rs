#![allow(clippy::all)]
use bevy::{
    asset::LoadState,
    sprite::{TextureAtlas, TextureAtlasBuilder},
    window::WindowMode,
};

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

mod math;
mod camera;
mod area;
mod generate;
mod generator;
mod controls;
mod gui;
mod selection;
mod state;
mod spectrum;

use state::State;
use area::{Area,Biome,Soil};

fn main() {
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
        .init_resource::<WarfareResources>()
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

        // set up controls plugin/system
        .add_plugin(controls::ControlsPlugin)
        .add_startup_system(controls::setup.system())
        
        // set up gui plugin/system
        .add_plugin(gui::GuiPlugin)
        .add_startup_system(gui::setup.system())

        // set up generator plugin/system
        .add_plugin(generate::GeneratorPlugin)
        .add_startup_system(generate::setup.system())

        .add_startup_system(setup.system())
        .run()
}

#[derive(Default, Clone)]
struct WarfareResources {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,

    loaded_textures: bool,
    loaded_fonts: bool,
}

pub struct DiagText;
pub struct OverText;

fn setup(
    mut _commands: Commands,
    mut state: ResMut<State>,
    mut resources: ResMut<WarfareResources>, 
    asset_server: Res<AssetServer>,
) {
    resources.textures = asset_server.load_folder("textures").unwrap();
    state.terrain.update = true; // force terrain generation
}