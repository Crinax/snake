use bevy::{
    asset::Assets,
    color::Color,
    ecs::{system::Command, world::World},
    math::{Vec2, primitives::Rectangle},
    prelude::Mut,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::snake::components::{SnakeSegment, SnakeSegmentPosition};

pub struct AddSnakeSegment;

impl Command for AddSnakeSegment {
    fn apply(self, world: &mut World) {
        let mut snake_query = world.query::<(&SnakeSegment, &SnakeSegmentPosition)>();

        let segments_count = snake_query.iter(world).count();

        let mut position = SnakeSegmentPosition::default();
        let mesh_handle = world.resource_scope(|_world, mut mushes: Mut<Assets<Mesh>>| {
            let rect = Rectangle::new(10.0, 10.0);
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
                position.0 = last_position.0 + Vec2::new(10.0, 0.0);
            }
        }

        world.spawn((
            SnakeSegment(segments_count),
            position,
            Mesh2d(mesh_handle),
            MeshMaterial2d(color_material),
            Transform::from_xyz(150.0, 150.0, 0.0),
        ));
    }
}
