use bevy::{
    color::Color,
    ecs::{query::QuerySingleError, system::Command, world::World},
    text::{Text2d, TextColor},
};

use super::components::Score;

pub struct SpawnScore;
pub struct AddToScore(pub i32);

impl Command for SpawnScore {
    fn apply(self, world: &mut World) {
        let mut score = world.query::<&Score>();

        match score.single(world) {
            Ok(_) => (),
            Err(err) => match err {
                QuerySingleError::NoEntities(_) => {
                    world.spawn((
                        Score::default(),
                        Text2d::new("Score: 0"),
                        TextColor(Color::WHITE),
                    ));
                }
                QuerySingleError::MultipleEntities(_) => panic!("Multiple score entities"),
            },
        }
    }
}

impl Command for AddToScore {
    fn apply(self, world: &mut World) {
        let mut score = world.query::<&mut Score>();

        match score.single_mut(world) {
            Ok(mut score) => score.0 += self.0,
            Err(err) => match err {
                QuerySingleError::NoEntities(_) => panic!("No score entity"),
                QuerySingleError::MultipleEntities(_) => panic!("Multiple score entities"),
            },
        }
    }
}
