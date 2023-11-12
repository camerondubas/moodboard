use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};

use crate::{events::AddPostItEvent, hold::Holdable, item::Item, theme::colors::Palette};

pub struct PostItPlugin;

impl Plugin for PostItPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_initial_post_its)
            .add_systems(Update, add_post_it);
    }
}

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
    for event in events.iter() {
        draw_post_it(
            &mut commands,
            Vec3::new(0., 0., 0.0),
            Palette::AMBER_200,
            event.0.as_str(),
        );
    }
}

fn draw_post_it(commands: &mut Commands, position: Vec3, color: Color, text: &str) {
    let size = Vec2::new(400., 420.);
    let text_style = TextStyle {
        font_size: 32.0,
        color: Palette::GRAY_700.with_a(0.8),
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
            Holdable,
            Item,
            Name::new("Post-it Note"),
        ))
        .with_children(|builder| {
            let z = Vec3::new(0., 0., 0.1);
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
                transform: Transform::from_translation(z),
                ..default()
            });
        });
}
