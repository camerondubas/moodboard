use bevy::text::{BreakLineOn, Text2dBounds};
use rand::seq::SliceRandom;

use crate::{
    events::AddItemEvent,
    item::ItemBundle,
    prelude::*,
    select::components::Selected,
    theme::{Theme, ThemeDidChange},
};

const POST_IT_SIZE: Vec2 = Vec2::new(400., 420.);
const POST_IT_STROKE_WIDTH: f32 = 5.0;
const POST_IT_STROKE_WIDTH_SELECTED: f32 = 10.0;

pub struct PostItPlugin;

impl Plugin for PostItPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                add_post_it,
                select_post_it,
                remove_select.after(select_post_it),
                post_it_theme_change,
            ),
        );
    }
}

#[derive(Component)]
pub struct PostIt;

#[derive(Component)]
pub struct PostItShadow;

#[derive(Component)]
pub struct PostItText;

fn add_post_it(mut commands: Commands, mut events: EventReader<AddItemEvent>, theme: Res<Theme>) {
    for event in events.read() {
        if let AddItemEvent::PostIt(text) = event {
            let color = theme
                .post_it_colors
                .choose(&mut rand::thread_rng())
                .unwrap();
            draw_post_it(
                &mut commands,
                &theme,
                Vec3::new(0., 0., 0.0),
                *color,
                text.as_str(),
            );
        }
    }
}

fn select_post_it(
    mut newly_selected_query: Query<&mut Stroke, (Added<Selected>, With<PostIt>)>,
    theme: Res<Theme>,
) {
    for mut stroke in newly_selected_query.iter_mut() {
        stroke.options = StrokeOptions::default().with_line_width(POST_IT_STROKE_WIDTH_SELECTED);
        stroke.color = theme.post_it_stroke_color;
    }
}

fn remove_select(
    mut removed: RemovedComponents<Selected>,
    mut post_it_query: Query<&mut Stroke, (With<PostIt>, Without<Selected>)>,
    theme: Res<Theme>,
) {
    for removed_entity in removed.read() {
        if let Ok(mut post_it_stroke) = post_it_query.get_mut(removed_entity) {
            post_it_stroke.options = StrokeOptions::default().with_line_width(POST_IT_STROKE_WIDTH);
            post_it_stroke.color = theme.post_it_stroke_color
        }
    }
}

pub(crate) fn draw_post_it(
    commands: &mut Commands,
    theme: &Theme,
    position: Vec3,
    color: Color,
    text: &str,
) {
    let text_style = TextStyle {
        font_size: 32.0,
        color: theme.default_text_color,
        ..Default::default()
    };

    commands
        .spawn((
            ItemBundle {
                fill: Fill::color(theme.default_bg_color),
                stroke: Stroke::new(theme.post_it_stroke_color, POST_IT_STROKE_WIDTH),
                shape: ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: POST_IT_SIZE,
                        ..Default::default()
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(position)),
                    ..Default::default()
                },
                ..Default::default()
            },
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
                Fill::color(theme.post_it_shadow_color),
                PostItShadow,
                Name::new("Post-it Note Shadow"),
            ));

            builder.spawn((
                Text2dBundle {
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: POST_IT_SIZE * 0.8,
                    },
                    text: Text {
                        sections: vec![TextSection::new(text, text_style.clone())],
                        alignment: TextAlignment::Left,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
                    ..default()
                },
                PostItText,
            ));
        });
}

fn post_it_theme_change(
    mut theme_event_reader: EventReader<ThemeDidChange>,
    mut post_it_query: Query<(&mut Stroke, &mut Fill), With<PostIt>>,
    mut post_it_text_query: Query<
        &mut Text,
        (With<PostItText>, Without<PostIt>, Without<PostItShadow>),
    >,
    mut post_it_shadow_query: Query<
        &mut Fill,
        (With<PostItShadow>, Without<PostIt>, Without<PostItText>),
    >,
) {
    for event in theme_event_reader.read() {
        let theme = &event.theme;
        for (mut stroke, mut fill) in &mut post_it_query {
            stroke.color = theme.post_it_stroke_color;
            stroke.options = StrokeOptions::default().with_line_width(POST_IT_STROKE_WIDTH);

            fill.color = theme.default_bg_color
        }

        for mut text in &mut post_it_text_query {
            text.sections[0].style.color = theme.default_text_color;
        }

        for mut fill in &mut post_it_shadow_query {
            fill.color = theme.post_it_shadow_color
        }
    }
}
