use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct Score(pub i32);
