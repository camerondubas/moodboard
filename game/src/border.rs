use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{events::ResizeEvent, theme::ThemeResource};

pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_border)
            .add_systems(Update, update_border_on_resize);
    }
}

#[derive(Component)]
struct WindowBorderLeft;

#[derive(Component)]
struct WindowBorderRight;

#[derive(Component)]
struct WindowBorderTop;

#[derive(Component)]
struct WindowBorderBottom;

fn draw_border(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
    theme: Res<ThemeResource>,
) {
    let window = window_query.single();
    let (width, height) = (window.width(), window.height());
    let border_width = 5.;
    let border_color = theme.0.slate.get_400();
    let border_material = materials.add(ColorMaterial::from(border_color));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(border_width, height)).into())
                .into(),
            material: border_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                -(width / 2.) + border_width / 2.,
                0.,
                0.,
            )),
            ..default()
        },
        WindowBorderLeft,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(border_width, height)).into())
                .into(),
            material: border_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                (width / 2.) - border_width / 2.,
                0.,
                0.,
            )),
            ..default()
        },
        WindowBorderRight,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(width, border_width)).into())
                .into(),
            material: border_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.,
                -(height / 2.) + border_width / 2.,
                0.,
            )),
            ..default()
        },
        WindowBorderTop,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(width, border_width)).into())
                .into(),
            material: border_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.,
                (height / 2.) - border_width / 2.,
                0.,
            )),
            ..default()
        },
        WindowBorderBottom,
    ));
}

fn update_border_on_resize(
    mut meshes: ResMut<Assets<Mesh>>,
    mut window_query: Query<&mut Window>,
    mut resize_event_reader: EventReader<ResizeEvent>,
    mut left_query: Query<
        (&mut Transform, &mut Mesh2dHandle),
        (
            With<WindowBorderLeft>,
            Without<WindowBorderRight>,
            Without<WindowBorderTop>,
            Without<WindowBorderBottom>,
        ),
    >,
    mut right_query: Query<
        (&mut Transform, &mut Mesh2dHandle),
        (
            With<WindowBorderRight>,
            Without<WindowBorderLeft>,
            Without<WindowBorderTop>,
            Without<WindowBorderBottom>,
        ),
    >,
    mut top_query: Query<
        (&mut Transform, &mut Mesh2dHandle),
        (
            With<WindowBorderTop>,
            Without<WindowBorderLeft>,
            Without<WindowBorderRight>,
            Without<WindowBorderBottom>,
        ),
    >,
    mut bottom_query: Query<
        (&mut Transform, &mut Mesh2dHandle),
        (
            With<WindowBorderBottom>,
            Without<WindowBorderLeft>,
            Without<WindowBorderRight>,
            Without<WindowBorderTop>,
        ),
    >,
) {
    for event in resize_event_reader.read() {
        let mut window = window_query.single_mut();
        window.resolution.set(event.width, event.height);

        let (mut left, mut left_mesh) = left_query.single_mut();
        let (mut right, mut right_mesh) = right_query.single_mut();
        let (mut top, mut top_mesh) = top_query.single_mut();
        let (mut bottom, mut bottom_mesh) = bottom_query.single_mut();

        let (width, height) = (window.width(), window.height());
        let border_width = 5.;

        left.translation = Vec3::new(-(width / 2.) + border_width / 2., 0., 0.);
        left_mesh.0 = meshes.add(shape::Quad::new(Vec2::new(border_width, height)).into());

        right.translation = Vec3::new((width / 2.) - border_width / 2., 0., 0.);
        right_mesh.0 = meshes.add(shape::Quad::new(Vec2::new(border_width, height)).into());

        top.translation = Vec3::new(0., -(height / 2.) + border_width / 2., 0.);
        top_mesh.0 = meshes.add(shape::Quad::new(Vec2::new(width, border_width)).into());

        bottom.translation = Vec3::new(0., (height / 2.) - border_width / 2., 0.);
        bottom_mesh.0 = meshes.add(shape::Quad::new(Vec2::new(width, border_width)).into());
    }
}
