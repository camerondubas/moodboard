pub mod colors;
use bevy::window::WindowThemeChanged;

use crate::events::ThemeEvent;
use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum Theme {
    Dark,
    Light,
}

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_theme_change, on_theme_change_system));
    }
}

#[derive(Component, Default)]
pub(crate) struct ThemeStyle {
    pub dark: ThemeOptions,
    pub light: ThemeOptions,
}

impl ThemeStyle {
    #[allow(dead_code)]
    pub fn new(dark: ThemeOptions, light: ThemeOptions) -> Self {
        Self { dark, light }
    }
}

#[derive(Default)]
pub struct ThemeOptions {
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub text: Option<Text>,
}

fn on_theme_change(
    mut commands: Commands,
    mut theme_event_reader: EventReader<ThemeEvent>,
    theme_style_query: Query<(Entity, &ThemeStyle)>,
    mut fill_query: Query<&mut Fill>,
    mut stroke_query: Query<&mut Stroke>,
    mut text_query: Query<&mut Text>,
) {
    for event in theme_event_reader.read() {
        let bg_color = match event.theme {
            Theme::Dark => Palette::SLATE_900,
            Theme::Light => Palette::SLATE_100,
        };

        commands.insert_resource(ClearColor(bg_color));

        for (entity, theme_style) in &theme_style_query {
            let target_theme = match event.theme {
                Theme::Light => &theme_style.light,
                Theme::Dark => &theme_style.dark,
            };

            if let Some(target_fill) = target_theme.fill {
                match fill_query.get_component_mut::<Fill>(entity) {
                    Ok(mut fill) => {
                        *fill = target_fill;
                    }
                    Err(_) => {
                        commands.entity(entity).insert(target_fill);
                    }
                }
            }

            if let Some(target_stroke) = target_theme.stroke {
                match stroke_query.get_component_mut::<Stroke>(entity) {
                    Ok(mut stroke) => {
                        *stroke = target_stroke;
                    }
                    Err(_) => {
                        commands.entity(entity).insert(target_stroke);
                    }
                }
            }

            if let Some(target_text) = &target_theme.text {
                match text_query.get_component_mut::<Text>(entity) {
                    Ok(mut text) => {
                        *text = target_text.clone();
                    }
                    Err(_) => {
                        commands.entity(entity).insert(target_text.clone());
                    }
                }
            }
        }
    }
}

fn on_theme_change_system(mut theme_event_reader: EventReader<WindowThemeChanged>) {
    for event in theme_event_reader.read() {
        info!("Theme changed: {:?}", event.theme);
    }
}
