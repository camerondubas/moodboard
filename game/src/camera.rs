use bevy_pancam::{PanCam, PanCamPlugin};

use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin)
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    // Set initial scale to allow for zooming in
    camera.projection.scale = 2.0;

    commands.spawn(camera).insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        // Set max scale in order to prevent the camera from zooming too far out
        max_scale: Some(4.),
        // Set min scale in order to prevent the camera from zooming too far in
        min_scale: 1.0,

        min_y: Some(-3000.),
        max_y: Some(3000.),
        max_x: Some(4000.),
        min_x: Some(-4000.),

        ..Default::default()
    });
}
