use bevy::{
    diagnostic::{
        DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    window::WindowResized,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
        ))
        .add_systems(Startup, display_debug)
        .add_systems(
            Update,
            (fps_counter, on_window_resize, cpu_counter, mem_counter),
        );
    }
}

#[derive(Component)]
struct CpuText;

#[derive(Component)]
struct MemText;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ResolutionText;

fn display_debug(mut commands: Commands, window_query: Query<&Window>) {
    let font_size = 28.0;
    let text_style = TextStyle {
        font_size,
        // font: asset_server.load("fonts/Segoe-UI.ttf"),
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
                    TextSection::new("CPU: ", text_style.clone()),
                    TextSection::new("0", text_style.clone()),
                ]),
                CpuText,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("MEM: ", text_style.clone()),
                    TextSection::new("0", text_style.clone()),
                ]),
                MemText,
            ));

            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Resolution: ", text_style.clone()),
                    TextSection::new(format!("{} x {}", width, height), text_style),
                ]),
                ResolutionText,
            ));
        });
}

fn fps_counter(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn cpu_counter(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<CpuText>>) {
    for mut text in &mut query {
        if let Some(cpu) = diagnostics.get(SystemInformationDiagnosticsPlugin::CPU_USAGE) {
            if let Some(value) = cpu.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn mem_counter(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<MemText>>) {
    for mut text in &mut query {
        if let Some(mem) = diagnostics.get(SystemInformationDiagnosticsPlugin::MEM_USAGE) {
            if let Some(value) = mem.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn on_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut Text, With<ResolutionText>>,
) {
    for event in resize_events.iter() {
        for mut text in &mut query {
            text.sections[1].value = format!("{} x {}", event.width, event.height);
        }
    }
}
