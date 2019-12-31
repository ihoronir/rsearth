use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, /*Read,*/ ReadStorage, System, SystemData, Entities,  World, WriteStorage},
};
use crate::earth::{Plant, Herbivore, HERBIVORE_REACHABLE_RANGE};


#[derive(SystemDesc)]
pub struct PlantHerbivore;

impl<'s> System<'s> for PlantHerbivore {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Plant>,
        WriteStorage<'s, Herbivore>,
        ReadStorage<'s, Transform>
    );

    fn run(
        &mut self,
        (
            entities,
            mut plants,
            mut herbivores,
            transforms
        ): Self::SystemData
    ) {
        // 食べられる Plant の Id, 食べる Herbivore の Id, 引き継ぐ Nutrition
        'herbivore_loop: for (_, herbivore, herbivore_transform) in (&*entities, &mut herbivores, &transforms).join() {
            for (plant_entity, plant, plant_transform) in (&*entities, &mut plants, &transforms).join() {
                let plant_x = plant_transform.translation().x;
                let plant_y = plant_transform.translation().y;
                let herbivore_x = herbivore_transform.translation().x;
                let herbivore_y = herbivore_transform.translation().y;

                let x_diff = plant_x - herbivore_x;
                let y_diff = plant_y - herbivore_y;

                if x_diff * x_diff + y_diff * y_diff < HERBIVORE_REACHABLE_RANGE * HERBIVORE_REACHABLE_RANGE {
                    herbivore.nutrition += plant.nutrition;
                    entities.delete(plant_entity).expect("Failed to delete plant.");

                    continue 'herbivore_loop;
                }
            }
        }
    }
}
