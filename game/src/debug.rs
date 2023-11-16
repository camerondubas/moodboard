use crate::prelude::*;
use crate::{events::ResizeEvent, hold::HoldInfo, item::ItemCounterResource, CursorWorldCoords};
use bevy::diagnostic::{
    DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, display_debug)
        .add_systems(
            Update,
            (
                fps_counter,
                frame_time,
                on_window_resize,
                cursor_position,
                item_counter,
                hold_info,
            ),
        );
    }
}

#[derive(Component)]
struct CursorText;

#[derive(Component)]
struct HeldText;

#[derive(Component)]
struct ItemCounterText;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct FrameTimeText;

#[derive(Component)]
struct ResolutionText;

fn display_debug(
    mut commands: Commands,
    window_query: Query<&Window>,
    _asset_server: Res<AssetServer>,
) {
    let font_size = 28.0;
    let text_style = TextStyle {
        font_size,
        // font: asset_server.load("fonts/font.ttf"),
        color: Color::rgb_u8(148, 163, 184),
        ..Default::default()
    };
    let window = window_query.single();
    let (width, height) = (window.width(), window.height());

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::End,
                width: Val::Px(500.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                margin: UiRect::all(Val::Px(15.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("FPS: ", text_style.clone()),
                    TextSection::new("0", text_style.clone()),
                ]),
                FpsText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Frame Time: ", text_style.clone()),
                    TextSection::new("0", text_style.clone()),
                ]),
                FrameTimeText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Cursor: ", text_style.clone()),
                    TextSection::new("0, 0", text_style.clone()),
                ]),
                CursorText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Item Counter: ", text_style.clone()),
                    TextSection::new("0", text_style.clone()),
                ]),
                ItemCounterText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Resolution: ", text_style.clone()),
                    TextSection::new(format!("{:.0} x {:.0}", width, height), text_style.clone()),
                ]),
                ResolutionText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Held: ", text_style.clone()),
                    TextSection::new("?", text_style),
                ]),
                HeldText,
            ));
        });
}

fn fps_counter(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}

fn frame_time(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FrameTimeText>>,
) {
    for mut text in &mut query {
        if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(value) = frame_time.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}

fn cursor_position(
    cursor_coords: Res<CursorWorldCoords>,
    mut query: Query<&mut Text, With<CursorText>>,
) {
    for mut text in &mut query {
        // Update the value of the second section
        text.sections[1].value = format!(
            "{:.0}, {:.0}",
            cursor_coords.0.x as i32, cursor_coords.0.y as i32
        );
    }
}

fn hold_info(hold_info: Res<HoldInfo>, mut query: Query<&mut Text, With<HeldText>>) {
    for mut text in &mut query {
        // Update the value of the second section
        if let Some(start_position) = hold_info.start_position {
            text.sections[1].value = format!("{:.0}, {:.0}", start_position.x, start_position.y);
        } else {
            text.sections[1].value = format!("None");
        }
    }
}

fn item_counter(
    item_counter: Res<ItemCounterResource>,
    mut query: Query<&mut Text, With<ItemCounterText>>,
) {
    for mut text in &mut query {
        // Update the value of the second section
        text.sections[1].value = format!("{:.0}", item_counter.0.get_count());
    }
}

fn on_window_resize(
    mut resize_event_reader: EventReader<ResizeEvent>,
    mut query: Query<&mut Text, With<ResolutionText>>,
) {
    for event in resize_event_reader.read() {
        for mut text in &mut query {
            text.sections[1].value = format!("{:.0} x {:.0}", event.width, event.height);
        }
    }
}
