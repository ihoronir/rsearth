use rand::Rng;
use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, /*Read, ReadStorage,*/ System, SystemData, World, WriteStorage},
};
use crate::earth::{Creature, Plant};


#[derive(SystemDesc)]
pub struct SolarSystem;

impl<'s> System<'s> for SolarSystem {

    type SystemData = (
        WriteStorage<'s, Creature>,
        WriteStorage<'s, Plant>,
    );

    fn run(
        &mut self,
        (
            mut creatures,
            mut plants,
        ): Self::SystemData
    ) {
        let mut rng = rand::thread_rng();

        for (creature, _) in (&mut creatures, &mut plants).join() {
            creature.nutrition += rng.gen_range(0, 10); 
        }
    }
}
