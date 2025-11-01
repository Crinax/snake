mod commands;
mod components;
mod resources;
mod systems;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
    math::Vec2,
};

use crate::snake::resources::SnakeSpeed;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(SnakeSpeed(Vec2::new(-0.33, 0.0)))
            .add_systems(Startup, systems::add_snake_segment)
            .add_systems(
                Update,
                (systems::move_snake, systems::add_snake_segment).chain(),
            );
    }
}
