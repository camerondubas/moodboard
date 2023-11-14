use bevy::render::primitives::Aabb;

use crate::hold::is_cursor_over;
use crate::hold::Held;
use crate::hold::Holdable;
use crate::prelude::*;
use crate::CursorWorldCoords;

const MAX_Z: f32 = 999.0;
const SELECT_BOX_COLOR: Color = Palette::BLUE_400;
const SELECT_BOX_STROKE_WIDTH: f32 = 2.0;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_selection_box, size_selection_box));
    }
}

#[derive(Component)]
pub struct SelectionBox {
    pub start: Vec2,
    pub end: Option<Vec2>,
}

#[derive(Component)]
pub struct Selectable;

fn size_selection_box(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_coords: Res<CursorWorldCoords>,
    mut selection_box_query: Query<(Entity, &GlobalTransform), (With<SelectionBox>, With<Path>)>,
) {
    if let Ok((entity, transform)) = selection_box_query.get_single_mut() {
        if !mouse_button_input.pressed(MouseButton::Left) {
            commands.entity(entity).despawn();
            return;
        }

        let distance = cursor_coords.0 - transform.translation().xy();
        let path = GeometryBuilder::build_as(&shapes::Rectangle {
            extents: distance.abs(),
            origin: get_anchor(distance),
        });

        commands.entity(entity).remove::<Path>();
        commands.entity(entity).insert(path);
    }
}

fn start_selection_box(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    holdable_query: Query<(&GlobalTransform, &Aabb), (With<Holdable>, Without<Held>)>,
    cursor_coords: Res<CursorWorldCoords>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let coords = cursor_coords.0;

        if holdable_query
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

fn get_anchor(position: Vec2) -> shapes::RectangleOrigin {
    match (position.x, position.y) {
        (x, y) if x > 0.0 && y > 0.0 => shapes::RectangleOrigin::BottomLeft,
        (x, y) if x < 0.0 && y > 0.0 => shapes::RectangleOrigin::BottomRight,
        (x, y) if x > 0.0 && y < 0.0 => shapes::RectangleOrigin::TopLeft,
        (x, y) if x < 0.0 && y < 0.0 => shapes::RectangleOrigin::TopRight,
        _ => shapes::RectangleOrigin::Center,
    }
}
