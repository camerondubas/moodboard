mod canvas;
mod debug;
pub mod events;
mod post_it;
pub mod theme;
mod ui;

use bevy::{
    pbr::Shadow,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    text::{BreakLineOn, Text2dBounds},
    window::WindowResolution,
};
use bevy_pancam::{PanCam, PanCamPlugin};
use canvas::CanvasPlugin;
use debug::DebugPlugin;
use events::{CounterEvent, Shared, SharedState};
use post_it::PostItPlugin;
use theme::{Theme, ThemePlugin, ThemeResource};
use ui::UiPlugin;

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
            CanvasPlugin,
            PanCamPlugin::default(),
            // Material2dPlugin::<CustomMaterial>::default(),
            event_plugin,
            DebugPlugin,
            ThemePlugin,
            UiPlugin,
            PostItPlugin,
        ))
        .insert_resource(SharedResource(shared_state))
        .add_systems(Startup, setup)
        .add_systems(Update, (punch_cube, toggle_key))
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}
#[derive(Resource)]
pub struct SharedResource(Shared<SharedState>);

#[derive(Component, Copy, Clone)]
pub struct Cube;

#[derive(Component, Copy, Clone)]
pub struct PostItNote;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // mut materials: ResMut<Assets<CustomMaterial>>,
    resource: Res<SharedResource>,
) {
    let mut camera = Camera2dBundle::default();
    // Set initial scale to allow for zooming in
    camera.projection.scale = 2.0;

    commands.spawn(camera).insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        // Set max scale in order to prevent the camera from zooming too far out
        max_scale: Some(10.),
        // Set min scale in order to prevent the camera from zooming too far in
        min_scale: 1.0,
        ..Default::default()
    });
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(20.).into()).into(),
            // material: materials.add(CustomMaterial {}),
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

fn toggle_key(mut query: Query<&mut PanCam>, keys: Res<Input<KeyCode>>) {
    // Space = Toggle Panning
    if keys.just_pressed(KeyCode::Space) {
        for mut pancam in &mut query {
            pancam.enabled = !pancam.enabled;
        }
    }
    // T = Toggle Zoom to Cursor
    if keys.just_pressed(KeyCode::T) {
        for mut pancam in &mut query {
            pancam.zoom_to_cursor = !pancam.zoom_to_cursor;
        }
    }
}
