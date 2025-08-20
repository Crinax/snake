use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::Commands};

pub(super) fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
