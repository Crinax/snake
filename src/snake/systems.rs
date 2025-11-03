use bevy::prelude::*;

use crate::snake::{
    SnakeAddSegmentTimer,
    commands::{
        AddSnakeSegment, MOVE_SNAKE_DOWN, MOVE_SNAKE_LEFT, MOVE_SNAKE_RIGHT, MOVE_SNAKE_UP,
    },
    components::{SnakeSegment, SnakeSpeed},
    resources::SnakeTimer,
};

pub(super) fn add_snake_segment(
    time: Res<Time>,
    mut timer: ResMut<SnakeAddSegmentTimer>,
    mut commands: Commands,
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.queue(AddSnakeSegment);
    }
}

pub(super) fn move_snake(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut snake: Query<(&mut Transform, &SnakeSpeed, &SnakeSegment)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut prev_position: Option<Vec3> = None;

        let first_segment = snake.iter_mut().find(|(_, _, snake)| snake.0 == 0);

        if let Some((mut transform, speed, _)) = first_segment {
            prev_position = Some(transform.translation);
            transform.translation.x += speed.0.x;
            transform.translation.y += speed.0.y;
            transform.translation.z += speed.0.z;
        }

        let sorted = snake
            .iter_mut()
            .sort_by::<&SnakeSegment>(|a, b| a.0.cmp(&b.0))
            .filter(|(_, _, snake)| snake.0 != 0);

        for (mut transform, _, _) in sorted {
            let prev = transform.translation.clone();

            if let Some(previous) = prev_position {
                transform.translation.x = previous.x;
                transform.translation.y = previous.y;
                transform.translation.z = previous.z;
            }

            prev_position = Some(prev);
        }
    }
}

pub(super) fn handle_key_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        commands.queue(MOVE_SNAKE_UP);
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        commands.queue(MOVE_SNAKE_DOWN);
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        commands.queue(MOVE_SNAKE_LEFT);
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        commands.queue(MOVE_SNAKE_RIGHT);
    }
}
