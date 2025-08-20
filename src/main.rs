mod score;

use bevy::prelude::*;

use score::ScorePlugin;

#[derive(Resource)]
struct ScoreTimer(Timer);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn increase_score(time: Res<Time>, mut timer: ResMut<ScoreTimer>, mut commands: Commands) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.queue(score::commands::AddToScore(10));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScorePlugin))
        .insert_resource(ScoreTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, increase_score)
        .run();
}
