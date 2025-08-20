mod systems;

use bevy::app::{Plugin, Startup};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, systems::spawn_camera);
    }
}
