use bevy::{ecs::{
    query::With,
    system::{Commands, Query, Res},
}, transform::components::Transform};

use crate::snake::{
    SnakeSpeed,
    commands::AddSnakeSegment,
    components::{SnakeSegment, SnakeSegmentPosition},
};

pub(super) fn add_snake_segment(mut commands: Commands) {
    commands.queue(AddSnakeSegment);
}

pub(super) fn move_snake(
    speed: Res<SnakeSpeed>,
    mut snake: Query<(&mut SnakeSegmentPosition, &mut Transform), With<SnakeSegment>>,
) {
    for (mut segment, mut transform) in snake.iter_mut() {
        // segment.0 += speed.0;
        transform.translation.x += speed.0.x;
        transform.translation.y += speed.0.y;
    }
}
