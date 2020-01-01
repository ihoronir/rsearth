use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, /*Read,*/ ReadStorage, System, SystemData, Entities,  World, WriteStorage},
};
use crate::earth::{Herbivore, Carnivore,  CARNIVORE_REACHABLE_RANGE};


#[derive(SystemDesc)]
pub struct HerbivoreCarnivore;

impl<'s> System<'s> for HerbivoreCarnivore {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Herbivore>,
        WriteStorage<'s, Carnivore>,
        ReadStorage<'s, Transform>
    );

    fn run(
        &mut self,
        (
            entities,
            mut herbivores,
            mut carnivores,
            transforms
        ): Self::SystemData
    ) {
        'herbivore_loop: for (herbivore_entity, herbivore, herbivore_transform) in (&*entities, &mut herbivores, &transforms).join() {
            for (_, carnivore, carnivore_transform) in (&*entities, &mut carnivores, &transforms).join() {
                let herbivore_x = herbivore_transform.translation().x;
                let herbivore_y = herbivore_transform.translation().y;
                let carnivore_x = carnivore_transform.translation().x;
                let carnivore_y = carnivore_transform.translation().y;

                let x_diff = herbivore_x - carnivore_x;
                let y_diff = herbivore_y - carnivore_y;

                if x_diff * x_diff + y_diff * y_diff < CARNIVORE_REACHABLE_RANGE * CARNIVORE_REACHABLE_RANGE {
                    carnivore.nutrition += herbivore.nutrition;
                    entities.delete(herbivore_entity).expect("Failed to delete a herbivore.");

                    continue 'herbivore_loop;
                }
            }
        }
    }
}
