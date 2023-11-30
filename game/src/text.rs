use bevy::text::{BreakLineOn, TextLayoutInfo};

use crate::{
    events::AddItemEvent,
    item::ItemBundle,
    prelude::*,
    theme::{Theme, ThemeDidChange},
    FontStack,
};

const TEXT_SELECTED_PADDING: Vec2 = Vec2::new(20., 20.);

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_textbox_size, on_theme_change, add_text));
    }
}

#[derive(Component)]
pub struct CanvasText;

#[derive(Component)]
pub struct CanvasTextText;

pub(crate) fn spawn_text(
    commands: &mut Commands,
    theme: &Theme,
    position: Vec3,
    text: impl Into<String>,
    size: f32,
    font: Handle<Font>,
) {
    let text_style = TextStyle {
        font: font,
        font_size: size,
        color: theme.default_text_color,
        ..Default::default()
    };

    commands
        .spawn((
            ItemBundle {
                fill: Fill::color(Palette::TRANSPARENT),
                shape: ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: Vec2::ZERO,
                        ..Default::default()
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(position)),
                    ..Default::default()
                },
                ..Default::default()
            },
            CanvasText,
            Name::new("Canvas Text"),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(text, text_style.clone())],
                        alignment: TextAlignment::Left,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                CanvasTextText,
                Name::new("Canvas Text Text"),
            ));
        });
}

fn update_textbox_size(
    mut commands: Commands,
    canvas_text_text_query: Query<
        (&TextLayoutInfo, &Parent),
        Or<(
            Added<CanvasTextText>,
            (With<CanvasTextText>, Changed<TextLayoutInfo>),
        )>,
    >,
) {
    for (text_info, parent) in canvas_text_text_query.iter() {
        if text_info.logical_size == Vec2::ZERO {
            continue;
        }

        let new_path: Path = GeometryBuilder::build_as(&shapes::Rectangle {
            extents: text_info.logical_size + TEXT_SELECTED_PADDING,
            ..Default::default()
        });
        commands.entity(parent.get()).remove::<Path>();
        commands.entity(parent.get()).insert(new_path);
    }
}

fn on_theme_change(
    mut theme_event_reader: EventReader<ThemeDidChange>,
    mut text_query: Query<&mut Text, With<CanvasTextText>>,
) {
    for event in theme_event_reader.read() {
        for mut text in text_query.iter_mut() {
            for section in text.sections.iter_mut() {
                section.style.color = event.theme.default_text_color
            }
        }
    }
}

fn add_text(
    mut commands: Commands,
    mut events: EventReader<AddItemEvent>,
    theme: Res<Theme>,
    font_stack: Res<FontStack>,
) {
    for event in events.read() {
        if let AddItemEvent::Text(value) = event {
            spawn_text(
                &mut commands,
                &theme,
                Vec3::new(0., 0., 0.0),
                value.clone(),
                font_stack.size.large,
                font_stack.body.regular().clone(),
            );
        }
    }
}
