#![allow(clippy::type_complexity)]
use bevy::render::primitives::Aabb;
use bevy_inspector_egui::egui::epaint::text::cursor;

use crate::hold::is_cursor_over;
use crate::prelude::*;
use crate::CursorWorldCoords;

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
                update_selected_box,
                remove_selected_box,
                move_selected_entities.after(update_selected_box),
                start_selection_box,
                size_selection_box,
                end_selection_box.after(size_selection_box),
            ),
        );
    }
}

#[derive(Component)]
pub struct SelectionBox {
    pub start: Vec2,
    pub end: Option<Vec2>,
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component, Default)]
pub struct SelectedRect {
    rect: Rect,
    initial_rect: Rect,
}

impl SelectedRect {
    fn new(rect: Rect) -> Self {
        Self {
            rect,
            initial_rect: rect,
        }
    }
}

#[derive(Component)]
pub struct Selected {
    pub start_position: Vec2,
}

fn size_selection_box(
    mut commands: Commands,
    cursor_coords: Res<CursorWorldCoords>,
    selectable_query: Query<(Entity, &GlobalTransform, &Aabb), With<Selectable>>,
    mut selection_box_query: Query<
        (Entity, &SelectionBox),
        (With<SelectionBox>, With<Path>, Without<Selectable>),
    >,
) {
    if let Ok((entity, selection_box)) = selection_box_query.get_single_mut() {
        let distance = cursor_coords.current - selection_box.start;
        let path = GeometryBuilder::build_as(&shapes::Rectangle {
            extents: distance.abs(),
            origin: get_anchor(distance),
        });

        commands.entity(entity).remove::<Path>();
        commands.entity(entity).insert(path);

        let start = selection_box.start;
        let end = cursor_coords.current;

        let selection_top_left = Vec2::new(start.x.min(end.x), start.y.max(end.y));
        let selection_bottom_right = Vec2::new(start.x.max(end.x), start.y.min(end.y));

        selectable_query.for_each(|(en, transform, aabb)| {
            // check if the entity is within the selection box
            let translation = transform.translation();
            let min_x = translation.x - aabb.half_extents.x;
            let max_x = translation.x + aabb.half_extents.x;
            let min_y = translation.y - aabb.half_extents.y;
            let max_y = translation.y + aabb.half_extents.y;

            let is_within_selection_box = max_x >= selection_top_left.x
                && selection_bottom_right.x >= min_x
                && max_y >= selection_bottom_right.y
                && selection_top_left.y >= min_y;

            if is_within_selection_box {
                commands.entity(en).insert(Selected {
                    start_position: translation.xy(),
                });
            } else {
                commands.entity(en).remove::<Selected>();
            }
        });
    }
}

fn start_selection_box(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    selectable_query: Query<(&GlobalTransform, &Aabb), With<Selectable>>,
    selected_rect_query: Query<&SelectedRect>,
    cursor_coords: Res<CursorWorldCoords>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let coords = cursor_coords.current;

        if let Ok(selected_rect) = selected_rect_query.get_single() {
            if selected_rect.rect.contains(coords) {
                return;
            }
        }

        if selectable_query
            .iter()
            .any(|(transform, aabb)| is_cursor_over(coords, transform.translation(), aabb))
        {
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
            SelectionBox {
                start: coords,
                end: None,
            },
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

fn update_selected_box(
    mut commands: Commands,
    new_selected_query: Query<Entity, Added<Selected>>,
    selected_query: Query<(&GlobalTransform, &Aabb), With<Selected>>,
    mut selected_rect_query: Query<(Entity, &mut SelectedRect, &mut Transform)>,
    removed: RemovedComponents<Selected>,
) {
    if new_selected_query.is_empty() && removed.is_empty() {
        return;
    };

    if let Ok((entity, mut selected_box, mut transform)) = selected_rect_query.get_single_mut() {
        if let Some(rect) = get_surrounding_rect(selected_query.iter().collect::<Vec<_>>()) {
            selected_box.rect = rect;
            selected_box.initial_rect = rect;
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

fn remove_selected_box(
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
    cursor_coords: Res<CursorWorldCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut selected_rect_query: Query<(&mut SelectedRect, &mut Transform)>,
    selection_box_query: Query<&SelectionBox>,
    // mut selected_query: Query<(&Selected, &mut Transform)>,
) {
    if mouse_button_input.pressed(MouseButton::Left) && selection_box_query.is_empty() {
        if let Ok((mut selected_rect, mut transform)) = selected_rect_query.get_single_mut() {
            if let Some(hold_start) = cursor_coords.hold_start {
                if selected_rect.initial_rect.contains(hold_start) {
                    let distance = cursor_coords.hold_distance();
                    let start = selected_rect.initial_rect.center();

                    transform.translation = (start + distance).extend(transform.translation.z);
                    selected_rect.rect = Rect::from_center_size(
                        transform.translation.xy(),
                        selected_rect.rect.size(),
                    );
                }
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        if let Ok((mut selected_rect, _)) = selected_rect_query.get_single_mut() {
            selected_rect.initial_rect = selected_rect.rect;
        }
    }
}

fn get_anchor(position: Vec2) -> shapes::RectangleOrigin {
    match (position.x, position.y) {
        (x, y) if x > 0.0 && y > 0.0 => shapes::RectangleOrigin::BottomLeft,
        (x, y) if x < 0.0 && y > 0.0 => shapes::RectangleOrigin::BottomRight,
        (x, y) if x > 0.0 && y < 0.0 => shapes::RectangleOrigin::TopLeft,
        (x, y) if x < 0.0 && y < 0.0 => shapes::RectangleOrigin::TopRight,
        _ => shapes::RectangleOrigin::Center,
    }
}

fn get_surrounding_rect(query: Vec<(&GlobalTransform, &Aabb)>) -> Option<Rect> {
    let mut rect_option: Option<Rect> = None;
    for (transform, aabb) in query {
        let rect =
            Rect::from_center_half_size(transform.translation().xy(), aabb.half_extents.xy());

        rect_option = Some(rect_option.map_or(rect, |r| r.union(rect)));
    }

    rect_option
}
