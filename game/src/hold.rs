use crate::prelude::*;
use crate::CursorWorldCoords;
use bevy::render::primitives::Aabb;

pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hold_while_clicked, move_held_entities));
    }
}

#[derive(Component)]
pub struct Holdable;

#[derive(Component)]
pub struct Held {
    /// The offset between the cursor and the center of the sprite when it was clicked.
    pub offset: Vec2,
}

fn hold_while_clicked(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_coords: Res<CursorWorldCoords>,
    holdable_query: Query<(Entity, &GlobalTransform, &Aabb), (With<Holdable>, Without<Held>)>,
    held_query: Query<Entity, With<Held>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut possible = vec![];
        // If this gets more complex, look into this package:
        // https://github.com/aevyrie/bevy_mod_picking/issues/7
        for (entity, transform, aabb) in holdable_query.iter() {
            let coords = cursor_coords.0;
            let translation = transform.translation();
            let sprite_x_position = translation.x;
            let sprite_y_position = translation.y;

            let half_width = aabb.half_extents.x;
            let half_height = aabb.half_extents.y;

            let x_range = sprite_x_position - half_width..sprite_x_position + half_width;
            let y_range = sprite_y_position - half_height..sprite_y_position + half_height;

            if x_range.contains(&coords.x) && y_range.contains(&coords.y) {
                let offset = Vec2::new(coords.x - sprite_x_position, coords.y - sprite_y_position);
                possible.push((entity, offset, translation.z));
            }
        }

        if possible.len() > 0 {
            possible.sort_by(|(_, _, a_z), (_, _, b_z)| b_z.partial_cmp(a_z).unwrap());
            let (entity, offset, _) = possible[0];
            commands.entity(entity).insert(Held { offset });
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        for entity in held_query.iter() {
            commands.entity(entity).remove::<Held>();
        }
    }
}

fn move_held_entities(
    cursor_coords: Res<CursorWorldCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut held_query: Query<(&Held, &mut Transform)>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        for (held, mut transform) in held_query.iter_mut() {
            let cursor = cursor_coords.0;

            transform.translation.x = cursor.x - held.offset.x;
            transform.translation.y = cursor.y - held.offset.y;
        }
    }
}
