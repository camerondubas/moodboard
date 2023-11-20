use bevy::text::{BreakLineOn, Text2dBounds};
use rand::seq::SliceRandom;

use crate::{
    events::AddPostItEvent,
    item::Item,
    prelude::*,
    select::components::{Selectable, Selected},
    theme::{ThemeOptions, ThemeStyle},
};

const POST_IT_LIGHT_COLORS: [Color; 6] = [
    Palette::AMBER_200,
    Palette::GREEN_300,
    Palette::PURPLE_300,
    Palette::BLUE_300,
    Palette::PINK_300,
    Palette::SLATE_400,
];

const POST_IT_DARK_COLORS: [Color; 6] = [
    Palette::AMBER_700,
    Palette::GREEN_600,
    Palette::PURPLE_600,
    Palette::BLUE_600,
    Palette::PINK_700,
    Palette::SLATE_600,
];

const POST_IT_SIZE: Vec2 = Vec2::new(400., 420.);
const POST_IT_STROKE_WIDTH: f32 = 5.0;
const POST_IT_STROKE_ALPHA: f32 = 0.7;
const POST_IT_STROKE_COLOR: Color = Color::BLACK;
const POST_IT_SHADOW_COLOR_LIGHT: Color = Palette::GRAY_600;
const POST_IT_SHADOW_COLOR_DARK: Color = Palette::GRAY_600;
const POST_IT_SHADOW_ALPHA_LIGHT: f32 = 0.6;
const POST_IT_SHADOW_ALPHA_DARK: f32 = 0.9;
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
        Vec3::new(-600., 300., 0.0),
        POST_IT_LIGHT_COLORS[0],
        POST_IT_DARK_COLORS[0],
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(0., 300., 0.0),
        POST_IT_LIGHT_COLORS[1],
        POST_IT_DARK_COLORS[1],
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(600., 300., 0.0),
        POST_IT_LIGHT_COLORS[2],
        POST_IT_DARK_COLORS[2],
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(-600., -300., 0.0),
        POST_IT_LIGHT_COLORS[3],
        POST_IT_DARK_COLORS[3],
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(0., -300., 0.0),
        POST_IT_LIGHT_COLORS[4],
        POST_IT_DARK_COLORS[4],
        text,
    );

    draw_post_it(
        &mut commands,
        Vec3::new(600., -300., 0.0),
        POST_IT_LIGHT_COLORS[5],
        POST_IT_DARK_COLORS[5],
        text,
    );
}

fn add_post_it(mut commands: Commands, mut events: EventReader<AddPostItEvent>) {
    for event in events.read() {
        let color = POST_IT_LIGHT_COLORS
            .choose(&mut rand::thread_rng())
            .unwrap();
        draw_post_it(
            &mut commands,
            Vec3::new(0., 0., 0.0),
            *color,
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

fn draw_post_it(
    commands: &mut Commands,
    position: Vec3,
    light_color: Color,
    dark_color: Color,
    text: &str,
) {
    let text_style_light = TextStyle {
        font_size: 32.0,
        color: Palette::GRAY_700.with_a(0.8),
        ..Default::default()
    };

    let text_style_dark = TextStyle {
        font_size: 32.0,
        color: Palette::GRAY_200.with_a(0.8),
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
            Stroke::new(
                POST_IT_STROKE_COLOR.with_a(POST_IT_STROKE_ALPHA),
                POST_IT_STROKE_WIDTH,
            ),
            ThemeStyle {
                dark: ThemeOptions {
                    fill: Some(Fill::color(dark_color)),
                    ..Default::default()
                },
                light: ThemeOptions {
                    fill: Some(Fill::color(light_color)),
                    ..Default::default()
                },
            },
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
                ThemeStyle {
                    dark: ThemeOptions {
                        fill: Some(Fill::color(
                            POST_IT_SHADOW_COLOR_DARK.with_a(POST_IT_SHADOW_ALPHA_DARK),
                        )),
                        ..Default::default()
                    },
                    light: ThemeOptions {
                        fill: Some(Fill::color(
                            POST_IT_SHADOW_COLOR_LIGHT.with_a(POST_IT_SHADOW_ALPHA_LIGHT),
                        )),
                        ..Default::default()
                    },
                },
                Name::new("Post-it Note Shadow"),
            ));

            builder.spawn((
                Text2dBundle {
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: POST_IT_SIZE * 0.8,
                    },

                    // ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
                    ..default()
                },
                ThemeStyle {
                    dark: ThemeOptions {
                        text: Some(Text {
                            sections: vec![TextSection::new(text, text_style_dark.clone())],
                            alignment: TextAlignment::Left,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                        }),
                        ..Default::default()
                    },
                    light: ThemeOptions {
                        text: Some(Text {
                            sections: vec![TextSection::new(text, text_style_light.clone())],
                            alignment: TextAlignment::Left,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                        }),
                        ..Default::default()
                    },
                },
            ));
        });
}
