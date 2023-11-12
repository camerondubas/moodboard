use bevy::prelude::*;

use crate::theme::colors::Palette;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ui_border);
    }
}

fn ui_border(mut commands: Commands) {
    let border_width = 5.;
    let border_color = Palette::SLATE_400;

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(border_width)),
                ..Default::default()
            },
            border_color: border_color.into(),
            ..Default::default()
        },
        Name::new("Window Border"),
    ));
}
