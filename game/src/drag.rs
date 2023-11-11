use bevy::{prelude::*, transform::commands};

use crate::CursorWorldCoords;

pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (drag_and_drop, dragging));
    }
}

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Dragging;

fn drag_and_drop(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_coords: Res<CursorWorldCoords>,
    draggable_query: Query<
        (Entity, &GlobalTransform, &Sprite),
        (With<Draggable>, Without<Dragging>),
    >,
    dragging_query: Query<Entity, With<Dragging>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // If this gets more complex, look into this package:
        // https://github.com/aevyrie/bevy_mod_picking/issues/7
        for (entity, transform, sprite) in draggable_query.iter() {
            let width = sprite.custom_size.unwrap().x;
            let height = sprite.custom_size.unwrap().y;
            let x = transform.translation().x;
            let y = transform.translation().y;

            let left = x - width / 2.;
            let right = x + width / 2.;
            let top = y + height / 2.;
            let bottom = y - height / 2.;

            if cursor_coords.0.x > left
                && cursor_coords.0.x < right
                && cursor_coords.0.y > bottom
                && cursor_coords.0.y < top
            {
                commands.entity(entity).insert(Dragging);
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        for entity in dragging_query.iter() {
            commands.entity(entity).remove::<Dragging>();
        }
    }
}

fn dragging(
    mut commands: Commands,
    cursor_coords: Res<CursorWorldCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    dragging_query: Query<(Entity, &GlobalTransform, &Sprite), With<Dragging>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        for (entity, transform, sprite) in dragging_query.iter() {
            // the offset is the difference between the cursor and the center of the sprite
            // let width = sprite.custom_size.unwrap().x;
            // let height = sprite.custom_size.unwrap().y;

            // let offset_x = cursor_coords.0.x - transform.translation().x - width / 2.;
            // let offset_y = cursor_coords.0.y - transform.translation().y - height / 2.;

            // let offset_x = cursor_coords.0.x - width / 2.;
            // let offset_y = cursor_coords.0.y - height / 2.;

            let offset_x = cursor_coords.0.x;
            let offset_y = cursor_coords.0.y;

            commands
                .entity(entity)
                .insert(Transform::from_translation(Vec3::new(
                    offset_x, offset_y, 0.,
                )));
        }
    }
}
