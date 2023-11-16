use crate::item::ItemCounterResource;
use crate::prelude::*;
use crate::select::{Selectable, Selected};
use crate::CursorWorldCoords;
use bevy::render::primitives::Aabb;

pub struct DragAndDropPlugin;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoldInfo>().add_systems(
            Update,
            (
                hold_entities,
                move_held_entities.after(hold_entities),
                release_held_entities.after(move_held_entities),
            ),
        );
    }
}

#[derive(Resource, Default, Debug, Reflect)]
pub struct HoldInfo {
    pub start_position: Option<Vec2>,
}

impl HoldInfo {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_holding(&self) -> bool {
        self.start_position.is_some()
    }

    pub fn distance(&self, cursor: Vec2) -> Vec2 {
        if let Some(start_position) = self.start_position {
            cursor - start_position
        } else {
            Vec2::ZERO
        }
    }
}

#[derive(Component)]
pub struct Held {
    /// The offset between the cursor and the center of the sprite when it was clicked.
    pub offset: Vec2,
    pub start: Vec2,
}

fn hold_entities(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_coords: Res<CursorWorldCoords>,
    mut hold_info: ResMut<HoldInfo>,
    mut item_counter: ResMut<ItemCounterResource>,
    selectable_query: Query<(Entity, &GlobalTransform, &Aabb), With<Selectable>>,
    mut unselected_query: Query<(Entity, &mut Transform), (With<Selectable>, Without<Selected>)>,
    selected_query: Query<(Entity), With<Selected>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut possible = vec![];
        // If this gets more complex, look into this package:
        // https://github.com/aevyrie/bevy_mod_picking/issues/7
        for (entity, transform, aabb) in selectable_query.iter() {
            let coords = cursor_coords.0;
            let translation = transform.translation();
            let is_cursor_over_selectable = is_cursor_over(coords, translation, aabb);

            if is_cursor_over_selectable {
                possible.push((entity, translation));
            }
        }

        if possible.len() > 0 {
            possible.sort_by(|(_, a), (_, b)| b.z.partial_cmp(&a.z).unwrap());
            let (entity, translation) = possible[0];

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

            // This shouldn't live here. It's not the concern of this system.
            // It should probably live in the Item system, and be triggered by
            // either an event or by a added/changed component.

            hold_info.start_position = Some(cursor_coords.0);
        }
    }
}

fn release_held_entities(
    mouse_button_input: Res<Input<MouseButton>>,
    mut hold_info: ResMut<HoldInfo>,
    mut selected_query: Query<(&mut Selected, &GlobalTransform)>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        hold_info.start_position = None;
        for (mut selected, transform) in selected_query.iter_mut() {
            selected.start_position = transform.translation().xy();
        }
    }
}

fn move_held_entities(
    cursor_coords: Res<CursorWorldCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    hold_info: Res<HoldInfo>,
    mut selected_query: Query<(&Selected, &mut Transform)>,
) {
    if mouse_button_input.pressed(MouseButton::Left) && hold_info.is_holding() {
        let distance = hold_info.distance(cursor_coords.0);

        for (selected, mut transform) in selected_query.iter_mut() {
            transform.translation =
                (selected.start_position + distance).extend(transform.translation.z);
        }
    }
}

pub fn is_cursor_over(coords: Vec2, translation: Vec3, aabb: &Aabb) -> bool {
    let half_width = aabb.half_extents.x;
    let half_height = aabb.half_extents.y;

    let x_range = translation.x - half_width..translation.x + half_width;
    let y_range = translation.y - half_height..translation.y + half_height;

    x_range.contains(&coords.x) && y_range.contains(&coords.y)
}
