mod commands;
mod components;
mod resources;
mod systems;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
    time::{Timer, TimerMode},
};

use crate::snake::resources::{SnakeAddSegmentTimer, SnakeTimer};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(SnakeTimer(Timer::from_seconds(0.25, TimerMode::Repeating)))
            .insert_resource(SnakeAddSegmentTimer(Timer::from_seconds(
                0.25,
                TimerMode::Repeating,
            )))
            .add_systems(Startup, systems::add_snake_segment)
            .add_systems(
                Update,
                (systems::move_snake, systems::add_snake_segment).chain(),
            )
            .add_systems(Update, systems::handle_key_input);
    }
}
