use bevy::prelude::*;

use crate::snake::components::{SnakeSegment, SnakeSpeed};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(in crate::snake) enum SegmentDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<SegmentDirection> for Vec3 {
    fn from(direction: SegmentDirection) -> Self {
        match direction {
            SegmentDirection::Up => Vec3::new(0.0, 20.0, 0.0),
            SegmentDirection::Down => Vec3::new(0.0, -20.0, 0.0),
            SegmentDirection::Left => Vec3::new(-20.0, 0.0, 0.0),
            SegmentDirection::Right => Vec3::new(20.0, 0.0, 0.0),
        }
    }
}

pub(in crate::snake) struct MoveSnakeSegment(pub(in crate::snake) SegmentDirection);

impl Command for MoveSnakeSegment {
    fn apply(self, world: &mut World) {
        let mut snake_query = world.query::<(&SnakeSegment, &mut SnakeSpeed)>();

        let snake_head = snake_query
            .iter_mut(world)
            .filter(|&(snake, _)| snake.0 == 0)
            .last();

        if let Some((_, mut snake_speed)) = snake_head {
            snake_speed.0 = self.0.into();
        }
    }
}

pub(in crate::snake) const MOVE_SNAKE_UP: MoveSnakeSegment = MoveSnakeSegment(SegmentDirection::Up);
pub(in crate::snake) const MOVE_SNAKE_DOWN: MoveSnakeSegment =
    MoveSnakeSegment(SegmentDirection::Down);
pub(in crate::snake) const MOVE_SNAKE_LEFT: MoveSnakeSegment =
    MoveSnakeSegment(SegmentDirection::Left);
pub(in crate::snake) const MOVE_SNAKE_RIGHT: MoveSnakeSegment =
    MoveSnakeSegment(SegmentDirection::Right);
