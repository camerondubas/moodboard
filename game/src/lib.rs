mod camera;
mod canvas;
mod color_swatch;
#[cfg(any(feature = "debug", rust_analyzer))]
mod debug;
pub mod events;
mod item;
mod post_it;
pub mod prelude;
mod select;
mod text;
pub mod theme;
mod ui;

use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
    window::WindowResolution,
};
use camera::CameraPlugin;
use canvas::CanvasPlugin;

use color_swatch::{random_color, spawn_swatch, ColorSwatchPlugin};
#[cfg(any(feature = "debug", rust_analyzer))]
use debug::DebugPlugin;
use events::{Shared, SharedState};
use item::ItemPlugin;
use post_it::{draw_post_it, PostItPlugin};
use prelude::*;
use select::SelectPlugin;
use text::{spawn_text, TextPlugin};
use theme::{Theme, ThemePlugin};
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
            CameraPlugin,
            // Material2dPlugin::<CustomMaterial>::default(),
            event_plugin,
            #[cfg(any(feature = "debug", rust_analyzer))]
            DebugPlugin,
            ThemePlugin,
            UiPlugin,
            ShapePlugin,
            PostItPlugin,
            ColorSwatchPlugin,
            TextPlugin,
            ItemPlugin,
            SelectPlugin,
        ))
        .add_systems(Startup, startup)
        .insert_resource(SharedResource(shared_state))
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

fn startup(mut commands: Commands, theme: Res<Theme>) {
    let text = "This is some default Text";

    draw_post_it(
        &mut commands,
        &theme,
        Vec3::new(-500., 200., 0.0),
        theme.post_it_colors[0],
        "This is a Post-It. \n\nYou can add more by clicking the chat bubble icon above.",
    );

    draw_post_it(
        &mut commands,
        &theme,
        Vec3::new(-64., -87., 0.0),
        theme.post_it_colors[1],
        "You can also add color swatches and text boxes.",
    );

    spawn_swatch(
        &mut commands,
        &theme,
        Vec3::new(-150., 283., 0.0),
        random_color(),
    );

    spawn_swatch(
        &mut commands,
        &theme,
        Vec3::new(286., -2., 0.0),
        random_color(),
    );

    spawn_swatch(
        &mut commands,
        &theme,
        Vec3::new(535., -2., 0.0),
        random_color(),
    );

    spawn_swatch(
        &mut commands,
        &theme,
        Vec3::new(-408., -170., 0.0),
        random_color(),
    );

    spawn_text(
        &mut commands,
        &theme,
        Vec3::new(350., 350., 0.0),
        "This is an example Moodboard".into(),
    );

    spawn_text(
        &mut commands,
        &theme,
        Vec3::new(511., -202., 0.0),
        "Try dragging things around!".into(),
    );
}
