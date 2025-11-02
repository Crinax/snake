use bevy::prelude::*;

use crate::snake::{
    commands::{
        AddSnakeSegment, MOVE_SNAKE_DOWN, MOVE_SNAKE_LEFT, MOVE_SNAKE_RIGHT, MOVE_SNAKE_UP,
    },
    components::{SnakeSegment, SnakeSpeed},
    resources::SnakeTimer,
};

pub(super) fn add_snake_segment(mut commands: Commands) {
    commands.queue(AddSnakeSegment);
}

pub(super) fn move_snake(
    time: Res<Time>,
    mut timer: ResMut<SnakeTimer>,
    mut snake: Query<(&mut Transform, &SnakeSpeed), With<SnakeSegment>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut transform, speed) in snake.iter_mut() {
            transform.translation.x += speed.0.x;
            transform.translation.y += speed.0.y;
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
