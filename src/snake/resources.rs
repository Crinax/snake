use bevy::{ecs::resource::Resource, time::Timer};

#[derive(Resource)]
pub(super) struct SnakeTimer(pub(super) Timer);
