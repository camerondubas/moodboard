use crate::{
    canvas::CursorCoords, events::ResizeEvent, item::ItemCounter, prelude::*, SharedResource,
};
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
                hold_distance,
            ),
        );
    }
}

#[derive(Component)]
struct CursorText;

#[derive(Component)]
struct HoldText;

#[derive(Component)]
struct ItemCounterText;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct FrameTimeText;

#[derive(Component)]
struct ResolutionText;

#[derive(Component)]
struct HoldDistanceText;

fn display_debug(
    mut commands: Commands,
    window_query: Query<&Window>,
    _asset_server: Res<AssetServer>,
    shared_resource: Res<SharedResource>,
) {
    let font_size = 16.0;
    let text_style = TextStyle {
        font_size,
        // font: asset_server.load("fonts/font.ttf"),
        color: Color::BLACK,
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
            let name = shared_resource.0.lock().unwrap().name.clone();
            parent.spawn(TextBundle::from_sections([TextSection::new(
                name,
                text_style.clone(),
            )]));
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
                    TextSection::new("Hold Info: ", text_style.clone()),
                    TextSection::new("?", text_style.clone()),
                ]),
                HoldText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Hold Distance: ", text_style.clone()),
                    TextSection::new("?", text_style.clone()),
                ]),
                HoldDistanceText,
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
    cursor_coords: Res<CursorCoords>,
    mut query: Query<&mut Text, With<CursorText>>,
) {
    for mut text in &mut query {
        // Update the value of the second section
        text.sections[1].value = format!(
            "{:.0}, {:.0}",
            cursor_coords.current.x as i32, cursor_coords.current.y as i32
        );
    }
}

fn hold_info(cursor_coords: Res<CursorCoords>, mut query: Query<&mut Text, With<HoldText>>) {
    for mut text in &mut query {
        // Update the value of the second section
        if let Some(start_position) = cursor_coords.hold_start {
            text.sections[1].value = format!("{:.0}, {:.0}", start_position.x, start_position.y);
        } else {
            text.sections[1].value = format!("None");
        }
    }
}

fn hold_distance(
    cursor_coords: Res<CursorCoords>,
    mut query: Query<&mut Text, With<HoldDistanceText>>,
) {
    for mut text in &mut query {
        // Update the value of the second section
        if cursor_coords.is_holding() {
            let distance = cursor_coords.hold_distance();
            text.sections[1].value = format!("{:.0}, {:.0}", distance.x, distance.y);
        } else {
            text.sections[1].value = format!("None");
        }
    }
}

fn item_counter(
    item_counter: Res<ItemCounter>,
    mut query: Query<&mut Text, With<ItemCounterText>>,
) {
    for mut text in &mut query {
        // Update the value of the second section
        text.sections[1].value = format!("{:.0}", item_counter.count());
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
