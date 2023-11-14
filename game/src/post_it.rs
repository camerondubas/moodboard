use crate::{prelude::*, select::Selected};
use bevy::text::{BreakLineOn, Text2dBounds};
use rand::seq::SliceRandom;

use crate::{events::AddPostItEvent, item::Item, select::Selectable};

const POST_IT_COLORS: [Color; 6] = [
    Palette::AMBER_200,
    Palette::GREEN_300,
    Palette::PURPLE_300,
    Palette::BLUE_300,
    Palette::PINK_300,
    Palette::SLATE_400,
];

const POST_IT_SIZE: Vec2 = Vec2::new(400., 420.);
const POST_IT_STROKE_WIDTH: f32 = 5.0;
const POST_IT_STROKE_ALPHA: f32 = 0.7;
const POST_IT_STROKE_COLOR: Color = Color::BLACK;
const POST_IT_SHADOW_COLOR: Color = Palette::GRAY_600;
const POST_IT_SHADOW_ALPHA: f32 = 0.6;
const POST_IT_STROKE_WIDTH_SELECTED: f32 = 10.0;

pub struct PostItPlugin;

impl Plugin for PostItPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_initial_post_its).add_systems(
            Update,
            (
                add_post_it,
                select_post_it,
                remove_select.after(select_post_it),
            ),
        );
    }
}

#[derive(Component)]
pub struct PostIt;

fn draw_initial_post_its(mut commands: Commands) {
    let text = "This is some default Text";

    draw_post_it(
        &mut commands,
        Vec3::new(-900., 300., 0.0),
        Palette::AMBER_200,
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(-300., 300., 0.0),
        Palette::GREEN_300,
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(300., 300., 0.0),
        Palette::PURPLE_300,
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(-900., -300., 0.0),
        Palette::BLUE_300,
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(-300., -300., 0.0),
        Palette::PINK_300,
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(300., -300., 0.0),
        Palette::SLATE_400,
        text,
    );
}

fn add_post_it(mut commands: Commands, mut events: EventReader<AddPostItEvent>) {
    for event in events.read() {
        let color = POST_IT_COLORS.choose(&mut rand::thread_rng()).unwrap();
        draw_post_it(
            &mut commands,
            Vec3::new(0., 0., 0.0),
            *color,
            event.0.as_str(),
        );
    }
}

fn select_post_it(mut newly_selected_query: Query<&mut Stroke, (Added<Selected>, With<PostIt>)>) {
    for mut stroke in newly_selected_query.iter_mut() {
        stroke.options = StrokeOptions::default().with_line_width(POST_IT_STROKE_WIDTH_SELECTED);
        stroke.color = POST_IT_STROKE_COLOR;
    }
}

fn remove_select(
    mut removed: RemovedComponents<Selected>,
    mut post_it_query: Query<&mut Stroke, (With<PostIt>, Without<Selected>)>,
) {
    for removed_entity in removed.read() {
        if let Ok(mut post_it_stroke) = post_it_query.get_mut(removed_entity) {
            post_it_stroke.options = StrokeOptions::default().with_line_width(POST_IT_STROKE_WIDTH);
            post_it_stroke.color = POST_IT_STROKE_COLOR.with_a(POST_IT_STROKE_ALPHA);
        }
    }
}

fn draw_post_it(commands: &mut Commands, position: Vec3, color: Color, text: &str) {
    let text_style = TextStyle {
        font_size: 32.0,
        color: Palette::GRAY_700.with_a(0.8),
        ..Default::default()
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: POST_IT_SIZE,
                    ..Default::default()
                }),
                spatial: SpatialBundle::from_transform(Transform::from_translation(position)),
                ..Default::default()
            },
            Fill::color(color),
            Stroke::new(
                POST_IT_STROKE_COLOR.with_a(POST_IT_STROKE_ALPHA),
                POST_IT_STROKE_WIDTH,
            ),
            Selectable,
            Item,
            PostIt,
            Name::new("Post-it Note"),
        ))
        .with_children(|builder| {
            builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: POST_IT_SIZE,
                        ..Default::default()
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        10., -10., -0.1,
                    ))),
                    ..Default::default()
                },
                Fill::color(POST_IT_SHADOW_COLOR.with_a(POST_IT_SHADOW_ALPHA)),
                Name::new("Post-it Note Shadow"),
            ));

            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(text, text_style.clone())],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: POST_IT_SIZE * 0.8,
                },

                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
                ..default()
            });
        });
}
