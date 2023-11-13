use bevy::{prelude::*, sprite::Anchor};

use crate::{theme::colors::Palette, CursorWorldCoords};

const MAX_Z: f32 = 999.0;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_selection_box
                //Temp disable, draw_stroke
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

#[derive(Component)]
pub struct Stroke;

#[derive(Component)]
pub struct StrokeChild;

fn draw_selection_box(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_coords: Res<CursorWorldCoords>,
    mut selection_box_query: Query<(Entity, &mut Sprite, &GlobalTransform), With<SelectionBox>>,
) {
    if let Ok((entity, mut sprite, transform)) = selection_box_query.get_single_mut() {
        if !mouse_button_input.pressed(MouseButton::Left) {
            commands.entity(entity).despawn();
            return;
        }

        let distance = cursor_coords.0 - transform.translation().xy();

        sprite.custom_size = Some(distance.abs());
        sprite.anchor = get_anchor(distance);
    } else {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Palette::BLUE_300.with_a(0.3),
                        custom_size: Some(Vec2::new(0.0, 0.0)),
                        anchor: Anchor::TopLeft,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(cursor_coords.0.x, cursor_coords.0.y, MAX_Z),
                    ..Default::default()
                },
                SelectionBox {
                    start: cursor_coords.0,
                    end: None,
                },
                Stroke,
                Name::new("Selection Box"),
            ));
        }
    }
}

#[allow(unused)]
fn draw_stroke(
    mut commands: Commands,
    mut stroke_query: Query<
        (Entity, &Sprite, &GlobalTransform),
        (With<Stroke>, Without<StrokeChild>),
    >,
    mut stroke_child_query: Query<
        (Entity, &Parent, &mut Transform, &mut Sprite),
        (With<StrokeChild>, Without<Stroke>),
    >,
) {
    for (entity, sprite, transform) in stroke_query.iter_mut() {
        // draw a black stroke around the border of the sprite
        let stroke_width = 200.0;
        let stroke_color = Color::BLACK;

        let size = sprite.custom_size.unwrap();
        let anchor_vec = sprite.anchor.as_vec();

        let offset_x = anchor_vec.x * size.x;
        let offset_y = anchor_vec.y * size.y;

        if let Ok((child_entity, parent, mut child_transform, mut child_sprite)) =
            stroke_child_query.get_single_mut()
        {
            if parent.get() == entity {
                // update child
                child_transform.translation = Vec3::new(-offset_x, -offset_y, 0.0);
                child_sprite.custom_size = Some(Vec2::new(size.x, size.y));
            }
        } else {
            // add child to entity
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: stroke_color,
                            custom_size: Some(Vec2::new(size.x, size.y)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(-100., -100., 0.1)),
                        ..Default::default()
                    },
                    StrokeChild,
                    Name::new("Stroke Child"),
                ));
            });
        }
    }
}

fn get_anchor(position: Vec2) -> Anchor {
    match (position.x, position.y) {
        (x, y) if x > 0.0 && y > 0.0 => Anchor::BottomLeft,
        (x, y) if x < 0.0 && y > 0.0 => Anchor::BottomRight,
        (x, y) if x > 0.0 && y < 0.0 => Anchor::TopLeft,
        (x, y) if x < 0.0 && y < 0.0 => Anchor::TopRight,
        _ => Anchor::Center,
    }
}
