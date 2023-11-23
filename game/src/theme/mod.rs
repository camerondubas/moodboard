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
    #[allow(dead_code)]
    pub mode: ThemeMode,
    pub theme: Theme,
}

#[derive(Resource, Clone)]
pub(crate) struct Theme {
    pub window_bg_color: Color,
    pub post_it_colors: [Color; 6],
    pub post_it_stroke_color: Color,
    pub post_it_shadow_color: Color,
    pub post_it_text_color: Color,
    pub debug_text_color: Color,
}

fn create_theme(mode: ThemeMode) -> Theme {
    match mode {
        ThemeMode::Light => Theme {
            window_bg_color: Palette::SLATE_100,
            post_it_colors: [
                Palette::AMBER_200,
                Palette::GREEN_300,
                Palette::PURPLE_300,
                Palette::BLUE_300,
                Palette::PINK_300,
                Palette::SLATE_400,
            ],
            post_it_stroke_color: Color::BLACK.with_a(0.7),
            post_it_shadow_color: Palette::GRAY_600.with_a(0.6),
            post_it_text_color: Palette::GRAY_700.with_a(0.8),
            debug_text_color: Palette::BLACK,
        },
        ThemeMode::Dark => Theme {
            window_bg_color: Palette::SLATE_900,
            post_it_colors: [
                Palette::AMBER_700,
                Palette::GREEN_600,
                Palette::PURPLE_600,
                Palette::BLUE_600,
                Palette::PINK_700,
                Palette::SLATE_600,
            ],
            post_it_stroke_color: Color::BLACK.with_a(0.7),
            post_it_shadow_color: Palette::GRAY_600.with_a(0.9),
            post_it_text_color: Palette::GRAY_200.with_a(0.8),
            debug_text_color: Palette::WHITE,
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
            mode: event.theme.clone(),
            theme: theme.clone(),
        });
    }
}

fn on_theme_change_system(mut theme_event_reader: EventReader<WindowThemeChanged>) {
    for event in theme_event_reader.read() {
        info!("Theme changed: {:?}", event.theme);
    }
}
