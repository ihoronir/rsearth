use rand::Rng;
use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, /*Read, ReadStorage,*/ System, SystemData, World, WriteStorage},
};
use crate::earth::Plant;


#[derive(SystemDesc)]
pub struct SolarSystem;

impl<'s> System<'s> for SolarSystem {

    type SystemData = (
        WriteStorage<'s, Plant>,
    );

    fn run(
        &mut self,
        (
            mut plants,
        ): Self::SystemData
    ) {
        let mut rng = rand::thread_rng();

        for plant in (&mut plants).join() {
            plant.nutrition += rng.gen_range(0, 10); 
        }
    }
}
