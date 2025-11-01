use bevy::{
    asset::Assets,
    color::Color,
    ecs::{system::Command, world::World},
    math::{Vec3, primitives::Rectangle},
    prelude::Mut,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::snake::components::SnakeSegment;

pub struct AddSnakeSegment;

impl Command for AddSnakeSegment {
    fn apply(self, world: &mut World) {
        let mut snake_query = world.query::<(&SnakeSegment, &Transform)>();

        let segments_count = snake_query.iter(world).count();

        let mut position = Vec3::default();
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
                position.x = last_position.local_x().x + 10.0;
                position.y = last_position.local_y().y;
                position.z = last_position.local_z().z;
            }
        }

        world.spawn((
            SnakeSegment(segments_count),
            Mesh2d(mesh_handle),
            MeshMaterial2d(color_material),
            Transform::from_translation(position),
        ));
    }
}
