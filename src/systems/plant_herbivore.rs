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
        // 食べられる Plant の Id, 食べる Herbivore の Id, 引き継ぐ Nutrition
        let mut diff: Vec<(u32, u32, u32)> = vec![];
        'plant_loop: for (plant_entity, _, plant_transform, plant_creature) in (&*entities, &mut plants, &transforms, &mut creatures).join() {
            for (herbivore_entity, _, herbivore_transform) in (&*entities, &mut herbivores, &transforms).join() {
                let plant_x = plant_transform.translation().x;
                let plant_y = plant_transform.translation().y;
                let herbivore_x = herbivore_transform.translation().x;
                let herbivore_y = herbivore_transform.translation().y;

                let x_diff = plant_x - herbivore_x;
                let y_diff = plant_y - herbivore_y;

                if x_diff * x_diff + y_diff * y_diff < HERBIVORE_REACHABLE_RANGE * HERBIVORE_REACHABLE_RANGE {
                    diff.push((plant_entity.id(), herbivore_entity.id(), plant_creature.nutrition)); 
                    continue 'plant_loop;
                }
            }
        }

        for (plant_id, herbivore_id, nutrition) in &diff {
            let plant = entities.entity(*plant_id);
            let herbivore_creature = creatures.get_mut(entities.entity(*herbivore_id));

            entities.delete(plant).expect("Failed to delete plant.");
            if let Some(herbivore_creature) = herbivore_creature {
                herbivore_creature.nutrition += nutrition;
            }
        }
    }
}
