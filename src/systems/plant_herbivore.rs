use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, /*Read,*/ ReadStorage, System, SystemData, Entities,  World, WriteStorage},
};
use crate::earth::{Creature, Plant, Herbivore, HERBIVORE_REACHABLE_RANGE};


#[derive(SystemDesc)]
pub struct PlantHerbivoreSystem;

impl<'s> System<'s> for PlantHerbivoreSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Creature>,
        WriteStorage<'s, Plant>,
        WriteStorage<'s, Herbivore>,
        ReadStorage<'s, Transform>
    );

    fn run(
        &mut self,
        (
            entities,
            mut creatures,
            mut plants,
            mut herbivores,
            transforms
        ): Self::SystemData
    ) {
        let mut herbivore_diff: Vec<()> = vec![];
        for (plant_entity, plant, plant_transform, plant_creature) in (&*entities, &mut plants, &transforms, &mut creatures).join() {
            for (herbivore_entity, herbivore, herbivore_transform) in (&*entities, &mut herbivores, &transforms).join() {
                let plant_x = plant_transform.translation().x;
                let plant_y = plant_transform.translation().y;
                let herbivore_x = herbivore_transform.translation().x;
                let herbivore_y = herbivore_transform.translation().y;

                let x_diff = plant_x - herbivore_x;
                let y_diff = plant_y - herbivore_y;

                if x_diff * x_diff + y_diff * y_diff < HERBIVORE_REACHABLE_RANGE * HERBIVORE_REACHABLE_RANGE {

                }
            }
        }
    }
}
