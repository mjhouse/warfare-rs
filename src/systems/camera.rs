use bevy::input::mouse::{MouseButton,MouseWheel,MouseMotion};
use bevy::prelude::*;

pub struct CameraPlugin;

pub struct Camera {
    /// the scale level (zoom)
    pub scale: f32,

    /// the speed of pan operations
    pub pan_speed: f32,

    /// the speed of zoom operations
    pub zoom_speed: f32,
    
    /// the position of the cursor
    pub position: Vec2,
    
    /// the button that triggers camera panning
	pub button: MouseButton,
}

impl Default for Camera {
	fn default() -> Self {
		Self {
            scale: 1.25,
            pan_speed: 1.75,
            zoom_speed: 0.25,
            position: Vec2::ZERO,
            button: MouseButton::Right,
		}
	}
}

fn camera_movement_system(
    inputs: Res<Input<MouseButton>>,
    mut scroll: EventReader<MouseWheel>,
    mut cursor: EventReader<CursorMoved>,
    mut motion: EventReader<MouseMotion>,
	mut queries: Query<(&mut Camera, &mut Transform)>,
) {
    let move_delta = motion.iter().fold(Vec2::ZERO,|a,e| a + e.delta);
    let scroll_delta = scroll.iter().fold(0.0,|a,e| a + e.y);
    let position = cursor.iter().last().map(|v| v.position);

    if !move_delta.is_nan() {
        for (mut camera, mut transform) in queries.iter_mut() {
            // if cursor position changed, update location
            if let Some(p) = position {
                camera.position = p;
            }

            if scroll_delta != 0.0 {
                camera.scale += scroll_delta * camera.zoom_speed;
                camera.scale = camera.scale.max(1.25);
                let s = camera.scale;

                transform.translation.x = (transform.translation.x / s).round() * s;
                transform.translation.y = (transform.translation.y / s).round() * s;
                transform.translation.z = (transform.translation.z / s).round() * s;
            }

            if inputs.pressed(camera.button) {
                let (axis_x,axis_y) = (move_delta.x,move_delta.y);
                let s = camera.scale;

                let x_value = axis_x * camera.pan_speed * s;
                let y_value = axis_y * camera.pan_speed * s;

                transform.translation += Vec3::new(-x_value, y_value, 0.0);

                transform.translation.x = (transform.translation.x / s).round() * s;
                transform.translation.y = (transform.translation.y / s).round() * s;
                transform.translation.z = (transform.translation.z / s).round() * s;
            }

            transform.scale.x = camera.scale;
            transform.scale.y = camera.scale;
            transform.scale.z = camera.scale;
        }
    }
}

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system(camera_movement_system.system());
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert_bundle(OrthographicCameraBundle::new_2d())
		.insert(Camera::default());
}