use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join,/* Read, ReadStorage,*/ System, SystemData, World, Entities, WriteStorage}
};
use crate::earth::{Creature};

#[derive(SystemDesc)]
pub struct CreatureSystem;

impl<'s> System<'s> for CreatureSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Creature>,
    );

    fn run(&mut self, (mut entities, mut creatures): Self::SystemData) {
        for (entity, creature) in (&*entities, &mut creatures).join() {
            if creature.life != 0 {
                creature.life -= 1;
            } else {
                entities.delete(entity);
            }
        }
    }
}

