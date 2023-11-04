use bevy::prelude::*;
use shared::{CounterEvent, Shared, SharedState};

pub fn run(event_plugin: impl Plugin, shared_state: Shared<SharedState>) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(event_plugin)
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    resource: Res<SharedResource>,
) {
    let name = resource.0.lock().unwrap().name.clone();
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Cube,
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(TextBundle::from_section(name, TextStyle::default()));
}

fn punch_cube(
    mut counter_event_reader: EventReader<CounterEvent>,
    mut cube_query: Query<&mut Transform, With<Cube>>,
) {
    let mut cube_transform = cube_query.get_single_mut().expect("no cube :(");
    let cube_offset = 0.5;
    for event in counter_event_reader.iter() {
        let y = ((event.value as f32) / 10.0) + cube_offset;
        cube_transform.translation = Vec3::new(0.0, y, 0.0);
    }
}
