pub mod commands;
mod components;
mod systems;

use bevy::app::{Plugin, Startup, Update};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, systems::spawn_score)
            .add_systems(Update, systems::update_score_view);
    }
}
