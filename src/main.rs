mod camera;
mod score;
mod snake;

use bevy::prelude::*;

use score::ScorePlugin;

use crate::{camera::CameraPlugin, snake::SnakePlugin};

#[derive(Resource)]
struct ScoreTimer(Timer);

fn increase_score(time: Res<Time>, mut timer: ResMut<ScoreTimer>, mut commands: Commands) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.queue(score::commands::AddToScore(10));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, ScorePlugin, SnakePlugin))
        .insert_resource(ScoreTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Update, increase_score)
        .run();
}
