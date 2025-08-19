use bevy::{
    ecs::system::{Commands, Query},
    text::Text2d,
};

use super::{commands::SpawnScore, components::Score};

pub(super) fn spawn_score(mut commands: Commands) {
    commands.queue(SpawnScore);
}

pub(super) fn update_score_view(mut query: Query<(&Score, &mut Text2d)>) {
    for (score, mut text) in query.iter_mut() {
        **text = format!("Score: {}", score.0);
    }
}
