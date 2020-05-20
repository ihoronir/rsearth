use crate::earth::Plant;
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage},
};
use rand::Rng;

#[derive(SystemDesc)]
pub struct SolarSystem;

impl<'s> System<'s> for SolarSystem {
    type SystemData = (WriteStorage<'s, Plant>,);

    fn run(&mut self, (mut plants,): Self::SystemData) {
        let mut rng = rand::thread_rng();

        for plant in (&mut plants).join() {
            plant.nutrition += rng.gen_range(0, 10);
        }
    }
}
