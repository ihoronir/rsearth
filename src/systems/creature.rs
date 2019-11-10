use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join,/* Read, ReadStorage,*/ System, SystemData, World, Entities, WriteStorage}
};
use crate::earth::{Creature, GROUND_WIDTH, GROUND_HEIGHT};

#[derive(SystemDesc)]
pub struct CreatureSystem;

impl<'s> System<'s> for CreatureSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Creature>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (entities, mut creatures, mut transforms): Self::SystemData) {
        for (entity, creature, transform) in (&*entities, &mut creatures, &mut transforms).join() {
            if creature.life != 0 {
                creature.life -= 1;

                let self_x = transform.translation().x;
                let self_y = transform.translation().y;

                if self_x < 0.0 {
                    transform.set_translation_x(GROUND_WIDTH + self_x); 
                }
                if GROUND_WIDTH < self_x {
                    transform.set_translation_x(self_x - GROUND_WIDTH); 
                }
                if self_y < 0.0 {
                    transform.set_translation_y(GROUND_HEIGHT + self_y); 
                }
                if GROUND_HEIGHT < self_y {
                    transform.set_translation_y(self_y - GROUND_HEIGHT); 
                }

            } else {
                entities.delete(entity).expect("Failed to delete creature.");
            }
        }
    }
}

