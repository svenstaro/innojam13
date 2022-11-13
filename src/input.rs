use bevy::{prelude::*, render::camera::RenderTarget};

use crate::{enemy::SpawnWaveEvent, gadget::SpawnCannonGadgetEvent, MainCamera};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(debug_keymap);
    }
}

pub fn get_world_cursor_pos(
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        Some(world_pos.truncate())
    } else {
        None
    }
}

fn debug_keymap(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut spawn_wave_events: EventWriter<SpawnWaveEvent>,
    mut spawn_cannon_events: EventWriter<SpawnCannonGadgetEvent>,
) {
    // Spawn next wave.
    if keys.just_pressed(KeyCode::N) {
        spawn_wave_events.send_default();
    }

    if mouse.just_pressed(MouseButton::Left) {
        spawn_cannon_events.send_default();
    }
}
