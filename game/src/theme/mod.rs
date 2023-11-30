pub mod colors;
use bevy::window::WindowThemeChanged;

use crate::events::ThemeEvent;
use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum ThemeMode {
    Dark,
    Light,
}

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        let theme = create_theme(ThemeMode::Light);
        app.insert_resource(theme)
            .add_event::<ThemeDidChange>()
            .add_systems(Update, (on_theme_change, on_theme_change_system));
    }
}

#[derive(Event)]
pub(crate) struct ThemeDidChange {
    pub theme: Theme,
}

#[derive(Resource, Clone)]
pub(crate) struct Theme {
    pub window_bg_color: Color,
    pub post_it_stroke_color: Color,
    pub post_it_shadow_color: Color,
    pub default_text_color: Color,
    pub debug_text_color: Color,
    pub color_swatch_text_color: Color,
    pub default_bg_color: Color,
}

fn create_theme(mode: ThemeMode) -> Theme {
    match mode {
        ThemeMode::Light => Theme {
            window_bg_color: Palette::SLATE_100,
            post_it_stroke_color: Color::BLACK.with_a(0.7),
            post_it_shadow_color: Palette::GRAY_600.with_a(0.6),
            default_text_color: Palette::GRAY_700.with_a(0.8),
            debug_text_color: Palette::BLACK,
            color_swatch_text_color: Palette::GRAY_700.with_a(0.8),
            default_bg_color: Palette::WHITE,
        },
        ThemeMode::Dark => Theme {
            window_bg_color: Palette::SLATE_900,
            post_it_stroke_color: Color::BLACK.with_a(0.7),
            post_it_shadow_color: Palette::GRAY_600.with_a(0.9),
            default_text_color: Palette::GRAY_200.with_a(0.8),
            debug_text_color: Palette::WHITE,
            color_swatch_text_color: Palette::GRAY_200.with_a(0.8),
            default_bg_color: Palette::SLATE_600,
        },
    }
}
fn on_theme_change(
    mut commands: Commands,
    mut theme_event_reader: EventReader<ThemeEvent>,
    mut theme: ResMut<Theme>,
    mut theme_did_change_writer: EventWriter<ThemeDidChange>,
) {
    for event in theme_event_reader.read() {
        let new_theme = create_theme(event.theme.clone());
        *theme = new_theme;
        commands.insert_resource(ClearColor(theme.window_bg_color.clone()));
        theme_did_change_writer.send(ThemeDidChange {
            theme: theme.clone(),
        });
    }
}

fn on_theme_change_system(mut theme_event_reader: EventReader<WindowThemeChanged>) {
    for event in theme_event_reader.read() {
        info!("Theme changed: {:?}", event.theme);
    }
}
