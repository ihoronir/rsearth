use crate::earth::{Herbivore, Plant, HERBIVORE_REACHABLE_RANGE};
use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct PlantHerbivore;

impl<'s> System<'s> for PlantHerbivore {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Plant>,
        WriteStorage<'s, Herbivore>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut plants, mut herbivores, transforms): Self::SystemData) {
        'plant_loop: for (plant_entity, plant, plant_transform) in
            (&*entities, &mut plants, &transforms).join()
        {
            for (_, herbivore, herbivore_transform) in
                (&*entities, &mut herbivores, &transforms).join()
            {
                let plant_x = plant_transform.translation().x;
                let plant_y = plant_transform.translation().y;
                let herbivore_x = herbivore_transform.translation().x;
                let herbivore_y = herbivore_transform.translation().y;

                let x_diff = plant_x - herbivore_x;
                let y_diff = plant_y - herbivore_y;

                if x_diff * x_diff + y_diff * y_diff
                    < HERBIVORE_REACHABLE_RANGE * HERBIVORE_REACHABLE_RANGE
                {
                    herbivore.nutrition += plant.nutrition;
                    entities
                        .delete(plant_entity)
                        .expect("Failed to delete a plant.");

                    continue 'plant_loop;
                }
            }
        }
    }
}
