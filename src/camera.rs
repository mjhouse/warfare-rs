use bevy::input::mouse::{MouseButton,MouseWheel,MouseMotion};
use bevy::prelude::*;

pub struct CameraPlugin;

pub struct Camera {
    /// the position of the cursor
    pub position: Vec2,
    /// the speed of zoom/pan operations
    pub speed: f32,
    /// the button that triggers camera panning
	pub button: MouseButton,
}

impl Default for Camera {
	fn default() -> Self {
		Self {
            position: Vec2::ZERO,
            speed: 1.75,
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
                // scale (zoom) view based on mouse wheel delta
                let z_value = scroll_delta * camera.speed;
                transform.scale += Vec3::new(z_value, z_value, z_value);

                // minimum values for zoom are 1.0
                if transform.scale.x < 1.0 { transform.scale.x = 1.0; }
                if transform.scale.y < 1.0 { transform.scale.y = 1.0; }
                if transform.scale.z < 1.0 { transform.scale.z = 1.0; }

                // let cx = camera.position.x / 100.0 * camera.speed * transform.scale.x;
                // let cy = camera.position.y / 100.0 * camera.speed * transform.scale.y;

                // transform.translation -= Vec3::new(cx, cy, 0.0);
            }

            if inputs.pressed(camera.button) {
                let (axis_x,axis_y) = (move_delta.x,move_delta.y);

                let x_value = axis_x * camera.speed * transform.scale.x;
                let y_value = axis_y * camera.speed * transform.scale.y;

                transform.translation += Vec3::new(-x_value, y_value, 0.0);
            }
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