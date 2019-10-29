use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join,/* Read, ReadStorage,*/ System, SystemData, World, Entities, WriteStorage}
};
use crate::earth::{Herbivore};

#[derive(SystemDesc)]
pub struct HerbivoreSystem;

impl<'s> System<'s> for HerbivoreSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Herbivore>,
    );

    fn run(&mut self, (mut entities, mut herbivores): Self::SystemData) {
        for (entity, herbivore) in (&*entities, &mut herbivores).join() {

        }
    }
}

