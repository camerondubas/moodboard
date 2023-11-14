use crate::prelude::*;
use crate::CursorWorldCoords;

const MAX_Z: f32 = 999.0;
const SELECT_BOX_COLOR: Color = Palette::BLUE_400;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_selection_box);
    }
}

#[derive(Component)]
pub struct SelectionBox {
    pub start: Vec2,
    pub end: Option<Vec2>,
}

#[derive(Component)]
pub struct Selectable;

fn draw_selection_box(
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
    } else {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: Vec2::new(0.0, 0.0),
                        origin: shapes::RectangleOrigin::TopLeft,
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(
                        cursor_coords.0.x,
                        cursor_coords.0.y,
                        MAX_Z,
                    )),
                    ..Default::default()
                },
                Fill::color(SELECT_BOX_COLOR.with_a(0.3)),
                Stroke::new(SELECT_BOX_COLOR, 2.0),
                SelectionBox {
                    start: cursor_coords.0,
                    end: None,
                },
                Name::new("Selection Box"),
            ));
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
