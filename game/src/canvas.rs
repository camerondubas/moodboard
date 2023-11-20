use bevy::window::PrimaryWindow;
use bevy_pancam::PanCam;

use crate::events::ResizeEvent;
use crate::prelude::*;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorCoords>()
            .add_systems(Update, (update_window_resolution, cursor_world_coords));
    }
}

#[derive(Resource, Default)]
pub(crate) struct CursorCoords {
    pub current: Vec2,
    pub hold_start: Option<Vec2>,
}

impl CursorCoords {
    pub fn is_holding(&self) -> bool {
        self.hold_start.is_some()
    }

    pub fn hold_distance(&self) -> Vec2 {
        if let Some(start_position) = self.hold_start {
            self.current - start_position
        } else {
            Vec2::ZERO
        }
    }
}

fn update_window_resolution(
    mut window_query: Query<&mut Window>,
    mut resize_event_reader: EventReader<ResizeEvent>,
) {
    for event in resize_event_reader.read() {
        let mut window = window_query.single_mut();
        window.resolution.set(event.width, event.height);
    }
}

fn cursor_world_coords(
    mut cursor_coords: ResMut<CursorCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<PanCam>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_coords.current = world_position;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        cursor_coords.hold_start = Some(cursor_coords.current);
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        cursor_coords.hold_start = None;
    }
}
