use bevy::prelude::*;

use crate::CursorWorldCoords;

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
    holdable_query: Query<(Entity, &GlobalTransform, &Sprite), (With<Holdable>, Without<Held>)>,
    held_query: Query<Entity, With<Held>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // If this gets more complex, look into this package:
        // https://github.com/aevyrie/bevy_mod_picking/issues/7
        for (entity, transform, sprite) in holdable_query.iter() {
            let coords = cursor_coords.0;
            let sprite_size = sprite
                .custom_size
                .expect("Sprite must have custom size to be holdable");
            let translation = transform.translation();
            let sprite_x_position = translation.x;
            let sprite_y_position = translation.y;

            let half_width = sprite_size.x / 2.;
            let half_height = sprite_size.y / 2.;

            let x_range = sprite_x_position - half_width..sprite_x_position + half_width;
            let y_range = sprite_y_position - half_height..sprite_y_position + half_height;

            if x_range.contains(&coords.x) && y_range.contains(&coords.y) {
                commands.entity(entity).insert(Held {
                    offset: Vec2::new(coords.x - sprite_x_position, coords.y - sprite_y_position),
                });
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        for entity in held_query.iter() {
            commands.entity(entity).remove::<Held>();
        }
    }
}

fn move_held_entities(
    mut commands: Commands,
    cursor_coords: Res<CursorWorldCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    held_query: Query<(Entity, &Held)>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        for (entity, held) in held_query.iter() {
            let cursor = cursor_coords.0;
            let updated_position =
                Vec3::new(cursor.x - held.offset.x, cursor.y - held.offset.y, 0.0);

            commands
                .entity(entity)
                .insert(Transform::from_translation(updated_position));
        }
    }
}
