use bevy::{ecs::component::Component, math::Vec2, transform::components::Transform};

#[derive(Component, Default)]
pub(super) struct SnakeSegment(pub usize);

#[derive(Component, Default)]
#[require(Transform)]
pub(super) struct SnakeSegmentPosition(pub Vec2);
