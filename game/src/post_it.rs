use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};

use crate::theme::{colors::ColorTheme, ThemeResource};

pub struct PostItPlugin;

impl Plugin for PostItPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_initial_post_its);
    }
}

fn draw_initial_post_its(mut commands: Commands, theme: Res<ThemeResource>) {
    let text = "This is some default Text";

    draw_post_it(
        &mut commands,
        &theme.0.clone(),
        Vec3::new(-450., 150., 0.),
        theme.0.amber.get_200(),
        text,
    );

    draw_post_it(
        &mut commands,
        &theme.0,
        Vec3::new(-150., 150., 0.),
        theme.0.green.get_300(),
        text,
    );

    draw_post_it(
        &mut commands,
        &theme.0,
        Vec3::new(150., 150., 0.),
        theme.0.purple.get_300(),
        text,
    );

    draw_post_it(
        &mut commands,
        &theme.0,
        Vec3::new(-450., -150., 0.),
        theme.0.blue.get_300(),
        text,
    );

    draw_post_it(
        &mut commands,
        &theme.0,
        Vec3::new(-150., -150., 0.),
        theme.0.pink.get_300(),
        text,
    );

    draw_post_it(
        &mut commands,
        &theme.0,
        Vec3::new(150., -150., 0.),
        theme.0.slate.get_400(),
        text,
    );
}

fn draw_post_it(
    commands: &mut Commands,
    theme: &ColorTheme,
    position: Vec3,
    color: Color,
    text: &str,
) {
    let size = Vec2::new(200., 210.);
    let text_style = TextStyle {
        font_size: 18.0,
        color: theme.gray.get_700().with_a(0.8),
        ..Default::default()
    };
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            Name::new("Post-it Note"),
        ))
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(text, text_style.clone())],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: size * 0.8,
                },

                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });
}