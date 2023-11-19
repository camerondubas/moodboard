#![allow(clippy::type_complexity)]
use crate::prelude::*;
use crate::CursorCoords;
use bevy::render::primitives::Aabb;

use self::components::Selectable;
use self::components::Selected;
use self::components::SelectedRect;
use self::components::SelectionBox;
use self::utils::get_anchor;
use self::utils::get_surrounding_rect;

pub mod components;
mod utils;

const MAX_Z: f32 = 999.0;
const SELECT_BOX_COLOR: Color = Palette::BLUE_400;
const SELECT_BOX_STROKE_WIDTH: f32 = 2.0;
const SELECTED_RECT_COLOR: Color = Palette::PURPLE_600;
const SELECTED_RECT_STROKE_WIDTH: f32 = 5.0;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                create_selected_rect,
                update_selected_rect,
                select_entities,
                move_selected_entities.after(update_selected_rect),
                remove_selected_rect.after(move_selected_entities),
                start_selection_box,
                size_selection_box,
                end_selection_box.after(size_selection_box),
                clear_selection,
            ),
        );
    }
}

fn clear_selection(
    mut commands: Commands,
    selectable_query: Query<Entity, Added<Selectable>>,
    mut selected_query: Query<Entity, With<Selected>>,
) {
    if selectable_query.is_empty() || selected_query.is_empty() {
        return;
    };

    for entity in selected_query.iter_mut() {
        commands.entity(entity).remove::<Selected>();
    }
}

fn size_selection_box(
    mut commands: Commands,
    cursor_coords: Res<CursorCoords>,
    selectable_query: Query<(Entity, &GlobalTransform, &Aabb), With<Selectable>>,
    mut selection_box_query: Query<
        (Entity, &SelectionBox),
        (With<SelectionBox>, With<Path>, Without<Selectable>),
    >,
) {
    if let Ok((entity, selection_box)) = selection_box_query.get_single_mut() {
        let distance = cursor_coords.hold_distance();
        let path = GeometryBuilder::build_as(&shapes::Rectangle {
            extents: distance.abs(),
            origin: get_anchor(distance),
        });

        commands.entity(entity).remove::<Path>();
        commands.entity(entity).insert(path);

        let start = selection_box.start;
        let end = cursor_coords.current;
        let selection_rect = Rect::from_corners(start, end);

        selectable_query.for_each(|(selectable_entity, transform, aabb)| {
            let translation = transform.translation();
            let selectable_rect =
                Rect::from_center_half_size(translation.xy(), aabb.half_extents.xy());

            if !selection_rect.intersect(selectable_rect).is_empty() {
                commands.entity(selectable_entity).insert(Selected {
                    start_position: translation.xy(),
                });
            } else {
                commands.entity(selectable_entity).remove::<Selected>();
            }
        });
    }
}

fn start_selection_box(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    selectable_query: Query<(&GlobalTransform, &Aabb), With<Selectable>>,
    selected_rect_query: Query<&SelectedRect>,
    cursor_coords: Res<CursorCoords>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let coords = cursor_coords.current;

        if let Ok(selected_rect) = selected_rect_query.get_single() {
            if selected_rect.contains(coords) {
                return;
            }
        }

        if selectable_query.iter().any(|(transform, aabb)| {
            Rect::from_center_size(transform.translation().xy(), aabb.half_extents.xy())
                .contains(coords)
        }) {
            return;
        }

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(0.0, 0.0),
                    origin: shapes::RectangleOrigin::TopLeft,
                }),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(
                    coords.x, coords.y, MAX_Z,
                )),
                ..Default::default()
            },
            Fill::color(SELECT_BOX_COLOR.with_a(0.3)),
            Stroke::new(SELECT_BOX_COLOR, SELECT_BOX_STROKE_WIDTH),
            SelectionBox { start: coords },
            Name::new("Selection Box"),
        ));
    }
}

fn end_selection_box(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selection_box_query: Query<Entity, (With<SelectionBox>, With<Path>, Without<Selectable>)>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if let Ok(entity) = selection_box_query.get_single_mut() {
            commands.entity(entity).despawn();
        }
    }
}

fn create_selected_rect(
    mut commands: Commands,
    newly_selected_query: Query<(&GlobalTransform, &Aabb), Added<Selected>>,
    selected_rect_query: Query<&mut SelectedRect>,
) {
    if newly_selected_query.is_empty() || !selected_rect_query.is_empty() {
        return;
    };

    if let Some(rect) = get_surrounding_rect(newly_selected_query.iter().collect::<Vec<_>>()) {
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(rect.width(), rect.height()),
                    origin: shapes::RectangleOrigin::Center,
                }),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(
                    rect.center().x,
                    rect.center().y,
                    MAX_Z,
                )),
                ..Default::default()
            },
            Stroke::new(SELECTED_RECT_COLOR, SELECTED_RECT_STROKE_WIDTH),
            SelectedRect::new(rect),
            Name::new("Selected Rect"),
        ));
    }
}

fn update_selected_rect(
    mut commands: Commands,
    new_selected_query: Query<Entity, Added<Selected>>,
    selected_query: Query<(&GlobalTransform, &Aabb), With<Selected>>,
    mut selected_rect_query: Query<(Entity, &mut SelectedRect, &mut Transform)>,
    removed: RemovedComponents<Selected>,
) {
    if new_selected_query.is_empty() && removed.is_empty() {
        return;
    };

    if let Ok((entity, mut selected_rect, mut transform)) = selected_rect_query.get_single_mut() {
        if let Some(rect) = get_surrounding_rect(selected_query.iter().collect::<Vec<_>>()) {
            selected_rect.update(rect);
            selected_rect.commit();
            let path = GeometryBuilder::build_as(&shapes::Rectangle {
                extents: Vec2::new(rect.width(), rect.height()),
                origin: shapes::RectangleOrigin::Center,
            });

            transform.translation = rect.center().extend(MAX_Z);

            commands.entity(entity).remove::<Path>();
            commands.entity(entity).insert(path);
        }
    }
}

fn remove_selected_rect(
    mut commands: Commands,
    selected_query: Query<Entity, With<Selected>>,
    mut selected_rect_query: Query<Entity, With<SelectedRect>>,
) {
    if selected_query.is_empty() {
        if let Ok(entity) = selected_rect_query.get_single_mut() {
            commands.entity(entity).despawn();
        }
    };
}

fn move_selected_entities(
    cursor_coords: Res<CursorCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_rect_query: Query<(&mut SelectedRect, &mut Transform), Without<Selected>>,
    selection_box_query: Query<&SelectionBox>,
    mut selected_query: Query<(&mut Selected, &mut Transform)>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if let Ok((mut selected_rect, _)) = selected_rect_query.get_single_mut() {
            selected_rect.commit();

            for (mut selected, transform) in selected_query.iter_mut() {
                selected.start_position = transform.translation.xy();
            }
        }

        return;
    }

    if mouse_button_input.pressed(MouseButton::Left) && selection_box_query.is_empty() {
        if let Ok((mut selected_rect, mut transform)) = selected_rect_query.get_single_mut() {
            if let Some(hold_start) = cursor_coords.hold_start {
                if selected_rect.initial_rect().contains(hold_start) {
                    let distance = cursor_coords.hold_distance();
                    let start = selected_rect.initial_point();

                    transform.translation = (start + distance).extend(transform.translation.z);
                    selected_rect.move_to(transform.translation.xy());
                }

                for (selected, mut transform) in selected_query.iter_mut() {
                    let distance = cursor_coords.hold_distance();
                    let start = selected.start_position;

                    transform.translation = (start + distance).extend(transform.translation.z);
                }
            }
        }
    }
}

fn select_entities(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_coords: ResMut<CursorCoords>,
    mut selectable_query: Query<
        (Entity, &GlobalTransform, &Aabb),
        (With<Selectable>, Without<Selected>),
    >,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut topmost_entity: Option<(Entity, Vec3)> = None;
        // If this gets more complex, look into this package:
        // https://github.com/aevyrie/bevy_mod_picking/issues/7
        for (entity, global_transform, aabb) in selectable_query.iter() {
            let translation = global_transform.translation();
            let is_cursor_over_selectable =
                Rect::from_center_half_size(translation.xy(), aabb.half_extents.xy())
                    .contains(cursor_coords.current);

            if is_cursor_over_selectable {
                if let Some((_, top_translation)) = topmost_entity {
                    if top_translation.z < translation.z {
                        topmost_entity = Some((entity, translation));
                    }
                } else {
                    topmost_entity = Some((entity, translation));
                }
            }
        }

        if let Some((topmost_entity, translation)) = topmost_entity {
            if let Ok((entity, _, _)) = selectable_query.get_mut(topmost_entity) {
                commands.entity(entity).insert(Selected {
                    start_position: translation.xy(),
                });
            }
        }
    }
}
