use crate::item::ItemCounterResource;
use crate::prelude::*;
use crate::select::components::{Selectable, Selected};
use crate::CursorCoords;
use bevy::render::primitives::Aabb;

pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                hold_entities,
                // move_held_entities.after(hold_entities),
                // release_held_entities.after(move_held_entities),
            ),
        );
    }
}

fn hold_entities(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_coords: ResMut<CursorCoords>,
    mut item_counter: ResMut<ItemCounterResource>,
    selectable_query: Query<(Entity, &GlobalTransform, &Aabb), With<Selectable>>,
    mut unselected_query: Query<(Entity, &mut Transform), (With<Selectable>, Without<Selected>)>,
    selected_query: Query<Entity, With<Selected>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut topmost: Option<(Entity, Vec3)> = None;
        // If this gets more complex, look into this package:
        // https://github.com/aevyrie/bevy_mod_picking/issues/7
        for (entity, transform, aabb) in selectable_query.iter() {
            let coords = cursor_coords.current;
            let translation = transform.translation();
            let is_cursor_over_selectable =
                Rect::from_center_half_size(translation.xy(), aabb.half_extents.xy())
                    .contains(coords);

            if is_cursor_over_selectable {
                if let Some((_, top_translation)) = topmost {
                    if top_translation.z < translation.z {
                        topmost = Some((entity, translation));
                    }
                } else {
                    topmost = Some((entity, translation));
                }
            }
        }

        if let Some((entity, translation)) = topmost {
            if let Ok((unselected_entity, mut transform)) = unselected_query.get_mut(entity) {
                let nothing_selected = selected_query.is_empty();
                commands.entity(unselected_entity).insert(Selected {
                    start_position: translation.xy(),
                });

                if nothing_selected {
                    item_counter.0.increment();
                    transform.translation.z = item_counter.0.get_count();
                }
            }
        }
    }
}

fn release_held_entities(
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_coords: ResMut<CursorCoords>,
    mut selected_query: Query<(&mut Selected, &GlobalTransform)>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        cursor_coords.hold_start = None;
        for (mut selected, transform) in selected_query.iter_mut() {
            selected.start_position = transform.translation().xy();
        }
    }
}

fn move_held_entities(
    cursor_coords: Res<CursorCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_query: Query<(&Selected, &mut Transform)>,
) {
    if mouse_button_input.pressed(MouseButton::Left) && cursor_coords.is_holding() {
        let distance = cursor_coords.hold_distance();

        for (selected, mut transform) in selected_query.iter_mut() {
            transform.translation =
                (selected.start_position + distance).extend(transform.translation.z);
        }
    }
}
