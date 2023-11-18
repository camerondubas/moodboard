mod canvas;
#[cfg(feature = "debug")]
mod debug;
pub mod events;
mod hold;
mod item;
mod post_it;
pub mod prelude;
mod select;
pub mod theme;
mod ui;

use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
    window::{PrimaryWindow, WindowResolution},
};
use bevy_pancam::{PanCam, PanCamPlugin};
use canvas::CanvasPlugin;

#[cfg(feature = "debug")]
use debug::DebugPlugin;
use events::{CounterEvent, Shared, SharedState};
use hold::DragAndDropPlugin;
use item::ItemPlugin;
use post_it::PostItPlugin;
use prelude::*;
use select::SelectPlugin;
use theme::ThemePlugin;
use ui::UiPlugin;

pub fn run(event_plugin: impl Plugin, shared_state: Shared<SharedState>) {
    let size = shared_state.lock().unwrap().window_size;

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
            PanCamPlugin,
            // Material2dPlugin::<CustomMaterial>::default(),
            event_plugin,
            #[cfg(feature = "debug")]
            DebugPlugin,
            ThemePlugin,
            UiPlugin,
            ShapePlugin,
            PostItPlugin,
            DragAndDropPlugin,
            ItemPlugin,
            SelectPlugin,
        ))
        .insert_resource(SharedResource(shared_state))
        .init_resource::<CursorWorldCoords>()
        .add_systems(Startup, setup)
        .add_systems(Update, (punch_cube, toggle_key, my_cursor_system))
        .run();
}

#[derive(Resource, Default)]
struct CursorWorldCoords {
    current: Vec2,
    hold_start: Option<Vec2>,
}

impl CursorWorldCoords {
    pub fn is_holding(&self) -> bool {
        self.hold_start.is_some()
    }

    pub fn hold_distance(&self) -> Vec2 {
        if let Some(start_position) = self.hold_start {
            self.current - start_position
        } else {
            Vec2::ZERO
        }
    }
}

fn my_cursor_system(
    mut cursor_coords: ResMut<CursorWorldCoords>,
    mouse_button_input: Res<Input<MouseButton>>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<PanCam>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_coords.current = world_position;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        cursor_coords.hold_start = Some(cursor_coords.current);
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        cursor_coords.hold_start = None;
    }
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
