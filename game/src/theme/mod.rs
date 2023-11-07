pub mod colors;
use bevy::prelude::*;

use crate::events::ThemeEvent;

use self::colors::ColorTheme;

#[derive(Clone, Debug)]
pub enum Theme {
    Dark,
    Light,
}

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ThemeResource(ColorTheme::new()))
            .add_systems(Update, on_theme_change);
    }
}

#[derive(Resource)]
pub struct ThemeResource(pub ColorTheme);

fn on_theme_change(
    mut commands: Commands,
    mut theme_event_reader: EventReader<ThemeEvent>,
    theme: Res<ThemeResource>,
) {
    for event in theme_event_reader.read() {
        let bg_color = match event.theme {
            Theme::Dark => theme.0.slate.get_900(),
            Theme::Light => theme.0.slate.get_200(),
        };

        commands.insert_resource(ClearColor(bg_color));
    }
}
