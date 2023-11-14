pub mod colors;
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
        app.add_systems(Update, on_theme_change);
    }
}

fn on_theme_change(mut commands: Commands, mut theme_event_reader: EventReader<ThemeEvent>) {
    for event in theme_event_reader.read() {
        let bg_color = match event.theme {
            Theme::Dark => Palette::SLATE_900,
            Theme::Light => Palette::SLATE_100,
        };

        commands.insert_resource(ClearColor(bg_color));
    }
}
