use bevy::text::{BreakLineOn, Text2dBounds};
use rand::seq::SliceRandom;

use crate::{
    events::AddItemEvent,
    item::ItemBundle,
    post_it::PostItShadow,
    prelude::*,
    theme::{Theme, ThemeDidChange},
    FontStack,
};

const SWATCH_SIZE: Vec2 = Vec2::new(220., 250.);
const SWATCH_COLOR_SECTION_SIZE: Vec2 = Vec2::new(215., 150.);
const SWATCH_STROKE_WIDTH: f32 = 5.0;
const SWATCH_COLORS: [Color; 22] = [
    Palette::SLATE_500,
    Palette::GRAY_500,
    Palette::ZINC_500,
    Palette::NEUTRAL_500,
    Palette::STONE_500,
    Palette::RED_500,
    Palette::ORANGE_500,
    Palette::AMBER_500,
    Palette::YELLOW_500,
    Palette::LIME_500,
    Palette::GREEN_500,
    Palette::EMERALD_500,
    Palette::TEAL_500,
    Palette::CYAN_500,
    Palette::SKY_500,
    Palette::BLUE_500,
    Palette::INDIGO_500,
    Palette::VIOLET_500,
    Palette::PURPLE_500,
    Palette::FUCHSIA_500,
    Palette::PINK_500,
    Palette::ROSE_500,
];

pub(crate) fn random_color() -> Color {
    *SWATCH_COLORS.choose(&mut rand::thread_rng()).unwrap()
}

pub struct ColorSwatchPlugin;

impl Plugin for ColorSwatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_swatch, on_theme_change));
    }
}

#[derive(Component)]
pub struct ColorSwatch;

#[derive(Component)]
pub struct ColorSwatchText;

pub(crate) fn spawn_swatch(
    commands: &mut Commands,
    theme: &Theme,
    font_stack: &FontStack,
    position: Vec3,
    color: Color,
) {
    let rgba = color.as_rgba_u8();
    let text = format!("#{:x?}{:x?}{:x?}", rgba[0], rgba[1], rgba[2]);

    let text_style = TextStyle {
        font: font_stack.body.clone(),
        font_size: 32.0,
        color: theme.default_text_color,
        ..Default::default()
    };

    commands
        .spawn((
            ItemBundle {
                stroke: Stroke::new(theme.post_it_stroke_color, SWATCH_STROKE_WIDTH),
                shape: ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: SWATCH_SIZE,
                        ..Default::default()
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(position)),
                    ..Default::default()
                },
                ..Default::default()
            },
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
            fill.color = event.theme.default_bg_color;
        }

        for mut text in text_query.iter_mut() {
            text.sections[0].style.color = event.theme.color_swatch_text_color;
        }
    }
}

fn add_swatch(
    mut commands: Commands,
    mut events: EventReader<AddItemEvent>,
    theme: Res<Theme>,
    font_stack: Res<FontStack>,
) {
    for event in events.read() {
        if let AddItemEvent::Swatch(_color) = event {
            // let color = Color::hex(color).unwrap();
            spawn_swatch(
                &mut commands,
                &theme,
                &font_stack,
                Vec3::new(0., 0., 0.0),
                random_color(),
            );
        }
    }
}
