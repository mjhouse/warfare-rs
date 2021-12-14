use bevy::prelude::*;

use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;

pub struct IconPlugin;

fn icon_system(windows: Res<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icons/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

impl Plugin for IconPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(icon_system.system()); // get world position of pointer
    }
}
