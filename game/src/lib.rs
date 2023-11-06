mod border;
mod debug;
pub mod shared;
pub mod theme;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};
use border::BorderPlugin;
use debug::DebugPlugin;
use shared::{CounterEvent, Shared, SharedState};
use theme::ThemePlugin;

pub fn run(event_plugin: impl Plugin, shared_state: Shared<SharedState>) {
    let size = shared_state.lock().unwrap().window_size.clone();

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#bevy".to_string()),
                    resolution: WindowResolution::new(size.0, size.1),
                    ..default()
                }),
                ..default()
            }),
            event_plugin,
            DebugPlugin,
            ThemePlugin,
            BorderPlugin,
        ))
        .insert_resource(SharedResource(shared_state))
        .add_systems(Startup, setup)
        .add_systems(Update, punch_cube)
        .run();
}

#[derive(Resource)]
pub struct SharedResource(Shared<SharedState>);

#[derive(Component, Copy, Clone)]
pub struct Cube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    resource: Res<SharedResource>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex("6b21a8").unwrap())),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
        Cube,
    ));

    let name = resource.0.lock().unwrap().name.clone();

    commands.spawn(
        TextBundle::from_section(
            name,
            TextStyle {
                font_size: 32.0,
                // font: asset_server.load("fonts/Segoe-UI.ttf"),
                // color: Color::hex("6b21a8").unwrap(),
                color: Color::rgb_u8(148, 163, 184),
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            left: Val::Px(25.0),
            ..Default::default()
        }),
    );
}

fn punch_cube(
    mut counter_event_reader: EventReader<CounterEvent>,
    mut cube_query: Query<&mut Transform, With<Cube>>,
) {
    let mut cube_transform = cube_query.get_single_mut().expect("no cube :(");
    let cube_offset = 0.5;
    for event in counter_event_reader.read() {
        let y = (event.value as f32) * 10. + cube_offset;
        cube_transform.translation = Vec3::new(0.0, y, 0.0);
    }
}
