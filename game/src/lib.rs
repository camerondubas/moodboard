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
    asset::AssetMetaCheck,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
    utils::HashMap,
    window::WindowResolution,
};
use camera::CameraPlugin;
use canvas::CanvasPlugin;

use color_swatch::{spawn_swatch, ColorSwatchPlugin, SWATCH_COLORS};
#[cfg(any(feature = "debug", rust_analyzer))]
use debug::DebugPlugin;
use events::{Shared, SharedState};
use item::ItemPlugin;
use post_it::{spawn_image, spawn_post_it, PostItPlugin};
use prelude::*;
use rand::seq::SliceRandom;
use select::SelectPlugin;
use text::{spawn_text, TextPlugin};
use theme::{Theme, ThemePlugin};
use ui::UiPlugin;

pub fn run(event_plugin: impl Plugin, shared_state: Shared<SharedState>) {
    let size = shared_state.lock().unwrap().window_size;

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AssetMetaCheck::Never)
        .init_resource::<ImageCache>()
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
        .init_resource::<FontStack>()
        .run();
}

#[derive(Resource, Default, Debug)]
pub(crate) struct ImageCache {
    pub images: HashMap<Handle<Image>, Entity>,
}

#[derive(Resource, Default, Debug)]
pub(crate) struct FontStack {
    pub body: FontFamily,
    pub title: FontFamily,
    pub size: FontSizeMap,
}

#[derive(Default, Debug)]
struct FontFamily {
    regular: Handle<Font>,
    bold: Option<Handle<Font>>,
    italic: Option<Handle<Font>>,
}

impl FontFamily {
    pub fn regular(&self) -> Handle<Font> {
        self.regular.clone()
    }

    pub fn bold(&self) -> Handle<Font> {
        self.bold.clone().unwrap_or_else(|| self.regular.clone())
    }

    pub fn italic(&self) -> Handle<Font> {
        self.italic.clone().unwrap_or_else(|| self.regular.clone())
    }
}

#[derive(Debug)]
pub struct FontSizeMap {
    pub xxsmall: f32,
    pub xsmall: f32,
    pub small: f32,
    pub medium: f32,
    pub large: f32,
    pub xlarge: f32,
    pub xxlarge: f32,
}

impl Default for FontSizeMap {
    fn default() -> Self {
        Self {
            xxsmall: 8.0 * 2.,
            xsmall: 12.0 * 2.,
            small: 16.0 * 2.,
            medium: 24.0 * 2.,
            large: 32.0 * 2.,
            xlarge: 48.0 * 2.,
            xxlarge: 64.0 * 2.,
        }
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

fn startup(
    mut commands: Commands,
    theme: Res<Theme>,
    mut font_stack: ResMut<FontStack>,
    asset_server: Res<AssetServer>,
    mut image_cache: ResMut<ImageCache>,
) {
    let playfair = FontFamily {
        regular: asset_server.load("fonts/playfair/PlayfairDisplay-Regular.ttf"),
        bold: Some(asset_server.load("fonts/playfair/PlayfairDisplay-Bold.ttf")),
        italic: Some(asset_server.load("fonts/playfair/PlayfairDisplay-Italic.ttf")),
    };

    let source_sans = FontFamily {
        regular: asset_server.load("fonts/source-sans/SourceSans3-Regular.ttf"),
        bold: Some(asset_server.load("fonts/source-sans/SourceSans3-Bold.ttf")),
        italic: Some(asset_server.load("fonts/source-sans/SourceSans3-Italic.ttf")),
    };

    font_stack.title = playfair;
    font_stack.body = source_sans;
    let mut swatch_colors = SWATCH_COLORS.choose_multiple(&mut rand::thread_rng(), 4);

    spawn_post_it(
        &mut commands,
        &theme,
        &font_stack,
        Vec3::new(-500., 200., 0.0),
        "This is a Post-It. \n\nYou can add more by clicking the chat bubble icon above.",
    );

    spawn_post_it(
        &mut commands,
        &theme,
        &font_stack,
        Vec3::new(-64., -87., 0.0),
        "You can also add color swatches and text boxes.",
    );

    spawn_swatch(
        &mut commands,
        &theme,
        &font_stack,
        Vec3::new(-150., 283., 0.0),
        swatch_colors.next().unwrap(),
    );

    spawn_swatch(
        &mut commands,
        &theme,
        &font_stack,
        Vec3::new(286., -2., 0.0),
        swatch_colors.next().unwrap(),
    );

    spawn_swatch(
        &mut commands,
        &theme,
        &font_stack,
        Vec3::new(535., -2., 0.0),
        swatch_colors.next().unwrap(),
    );

    spawn_swatch(
        &mut commands,
        &theme,
        &font_stack,
        Vec3::new(-408., -170., 0.0),
        swatch_colors.next().unwrap(),
    );

    spawn_text(
        &mut commands,
        &theme,
        Vec3::new(400., 370., 0.0),
        "An Example Moodboard",
        font_stack.size.xlarge,
        font_stack.title.bold(),
    );

    spawn_text(
        &mut commands,
        &theme,
        Vec3::new(257., 280., 0.0),
        "Try dragging things around!",
        font_stack.size.large,
        font_stack.body.italic(),
    );

    let image = asset_server.load("images/night_lights.jpg");

    spawn_image(
        &mut commands,
        &theme,
        Vec3::new(576., -424., 0.0),
        image,
        &mut image_cache,
    );
}
