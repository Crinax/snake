use bevy::{ecs::resource::Resource, math::Vec2};

#[derive(Resource, Default)]
pub(super) struct SnakeSpeed(pub Vec2);
