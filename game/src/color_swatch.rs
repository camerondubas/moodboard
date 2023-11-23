use bevy::text::{BreakLineOn, Text2dBounds};

use crate::{
    item::Item,
    post_it::PostItShadow,
    prelude::*,
    select::components::Selectable,
    theme::{Theme, ThemeDidChange},
};

const SWATCH_SIZE: Vec2 = Vec2::new(220., 250.);
const SWATCH_COLOR_SECTION_SIZE: Vec2 = Vec2::new(215., 150.);
const SWATCH_STROKE_WIDTH: f32 = 5.0;

pub struct ColorSwatchPlugin;

impl Plugin for ColorSwatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_initial_swatches)
            .add_systems(Update, on_theme_change);
    }
}

#[derive(Component)]
pub struct ColorSwatch;

#[derive(Component)]
pub struct ColorSwatchText;

fn draw_initial_swatches(mut commands: Commands, theme: Res<Theme>) {
    spawn_swatch(
        &mut commands,
        &theme,
        Vec3::new(1000., 300., 0.0),
        Palette::RED_500,
    );
    spawn_swatch(
        &mut commands,
        &theme,
        Vec3::new(1000., 0., 0.0),
        Palette::GREEN_500,
    );
    spawn_swatch(
        &mut commands,
        &theme,
        Vec3::new(1000., -300., 0.0),
        Palette::BLUE_500,
    );
}

fn spawn_swatch(commands: &mut Commands, theme: &Res<Theme>, position: Vec3, color: Color) {
    let rgba = color.as_rgba_u8();
    let text = format!("#{:x?}{:x?}{:x?}", rgba[0], rgba[1], rgba[2]);

    let text_style = TextStyle {
        font_size: 32.0,
        color: theme.post_it_text_color,
        ..Default::default()
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: SWATCH_SIZE,
                    ..Default::default()
                }),
                spatial: SpatialBundle::from_transform(Transform::from_translation(position)),
                ..Default::default()
            },
            Stroke::new(theme.post_it_stroke_color, SWATCH_STROKE_WIDTH),
            Fill::color(Palette::WHITE),
            Selectable,
            Item,
            ColorSwatch,
            Name::new("Swatch"),
        ))
        .with_children(|builder| {
            builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: SWATCH_COLOR_SECTION_SIZE,
                        ..Default::default()
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        0.,
                        (SWATCH_SIZE.y / 2.) - (SWATCH_COLOR_SECTION_SIZE.y / 2.) - 2.,
                        0.1,
                    ))),
                    ..Default::default()
                },
                Fill::color(color),
                Name::new("Swatch Color"),
            ));

            builder.spawn((
                Text2dBundle {
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: Vec2 {
                            x: SWATCH_SIZE.x,
                            y: SWATCH_SIZE.y / 2.,
                        } * 0.8,
                    },
                    text: Text {
                        sections: vec![TextSection::new(text, text_style.clone())],
                        alignment: TextAlignment::Left,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::new(0., -73., 0.1)),
                    ..default()
                },
                ColorSwatchText,
                Name::new("Swatch Text"),
            ));

            builder.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: SWATCH_SIZE,
                        ..Default::default()
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        10., -10., -0.1,
                    ))),
                    ..Default::default()
                },
                Fill::color(theme.post_it_shadow_color),
                PostItShadow,
                Name::new("Swatch Shadow"),
            ));
        });
}

fn on_theme_change(
    mut theme_event_reader: EventReader<ThemeDidChange>,
    mut fill_query: Query<&mut Fill, (With<ColorSwatch>, Without<ColorSwatchText>)>,
    mut text_query: Query<&mut Text, (With<ColorSwatchText>, Without<ColorSwatch>)>,
) {
    for event in theme_event_reader.read() {
        for mut fill in fill_query.iter_mut() {
            fill.color = event.theme.color_swatch_bg_color;
        }

        for mut text in text_query.iter_mut() {
            text.sections[0].style.color = event.theme.color_swatch_text_color;
        }
    }
}
