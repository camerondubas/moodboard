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
pub mod theme;
mod ui;

use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
    window::WindowResolution,
};
use camera::CameraPlugin;
use canvas::CanvasPlugin;

use color_swatch::ColorSwatchPlugin;
#[cfg(any(feature = "debug", rust_analyzer))]
use debug::DebugPlugin;
use events::{Shared, SharedState};
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
            ItemPlugin,
            SelectPlugin,
        ))
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
