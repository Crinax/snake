use bevy::prelude::*;

use crate::snake::{
    commands::{MOVE_SNAKE_LEFT, move_snake_segment::SegmentDirection},
    components::{SnakeSegment, SnakeSpeed},
};

pub(in crate::snake) struct AddSnakeSegment;

impl Command for AddSnakeSegment {
    fn apply(self, world: &mut World) {
        let mut snake_query = world.query::<(&SnakeSegment, &Transform)>();

        let segments_count = snake_query.iter(world).count();

        let mut position = Vec3::default();
        let mesh_handle = world.resource_scope(|_world, mut mushes: Mut<Assets<Mesh>>| {
            let rect = Rectangle::new(20.0, 20.0);
            mushes.add(rect)
        });
        let color_material =
            world.resource_scope(|_world, mut materials: Mut<Assets<ColorMaterial>>| {
                let blue = Color::srgb(0.0, 0.0, 1.0);
                materials.add(blue)
            });

        if segments_count > 0 {
            let last_snake_segment = snake_query
                .iter(world)
                .find(|&(snake, _)| snake.0 == segments_count - 1);

            if let Some((_, last_position)) = last_snake_segment {
                position = last_position.translation.clone();
            }
        }

        world.spawn((
            SnakeSegment(segments_count),
            SnakeSpeed(SegmentDirection::Left.into()),
            Mesh2d(mesh_handle),
            MeshMaterial2d(color_material),
            Transform::from_translation(position),
        ));
    }
}
