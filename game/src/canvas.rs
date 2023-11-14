use crate::events::ResizeEvent;
use crate::prelude::*;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_window_resolution);
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
