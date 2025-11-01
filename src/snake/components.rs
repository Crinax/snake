use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub(super) struct SnakeSegment(pub usize);
