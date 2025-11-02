use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Default)]
pub(super) struct SnakeSegment(pub usize);

#[derive(Component, Default)]
pub(super) struct SnakeSpeed(pub Vec3);
