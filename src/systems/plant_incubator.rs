use crate::earth::{
    Acceleration, Plant, Velocity, PLANT_INITIAL_NUTRITION, PLANT_MAX_LIFE, PLANT_MAX_SPEED,
    PLANT_MIN_LIFE, PLANT_SEPARATION_DISTANCE,
};
use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, World, WriteStorage},
    renderer::SpriteRender,
};
use rand::Rng;
use std::f32::consts::PI;

#[derive(SystemDesc)]
pub struct PlantIncubator;

impl<'s> System<'s> for PlantIncubator {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
        WriteStorage<'s, Plant>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut velocities,
            mut accelerations,
            mut plants,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();
        let mut new_plants: Vec<(Transform, SpriteRender)> = vec![];

        let mut plant_num = 0;

        for (entity, plant, transform, sprite_render) in
            (&*entities, &mut plants, &transforms, &sprite_renders).join()
        {
            plant_num += 1;
            if plant.life == 0 {
                entities.delete(entity).expect("Failed to delete a plant.");
            } else {
                plant.life -= 1;

                if plant.nutrition >= PLANT_INITIAL_NUTRITION * 2 {
                    let difference = {
                        let r = PI * 2.0 * rng.gen::<f32>();
                        let s = PLANT_SEPARATION_DISTANCE * (rng.gen::<f32>().sqrt());
                        (s * r.cos(), s * r.sin())
                    };
                    let mut new_transform = Transform::default();
                    new_transform.set_translation_xyz(
                        transform.translation().x + difference.0,
                        transform.translation().y + difference.1,
                        0.0,
                    );
                    let new_sprite_render = sprite_render;
                    new_plants.push((new_transform, new_sprite_render.clone()));

                    plant.nutrition -= PLANT_INITIAL_NUTRITION;
                }
            }
        }
        print!("{} ", plant_num);

        for (transform, sprite_render) in new_plants {
            entities
                .build_entity()
                .with(sprite_render, &mut sprite_renders)
                .with(Velocity { x: 0.0, y: 0.0 }, &mut velocities)
                .with(
                    Acceleration {
                        max_speed: PLANT_MAX_SPEED,
                        x: 0.0,
                        y: 0.0,
                    },
                    &mut accelerations,
                )
                .with(
                    Plant {
                        life: rng.gen_range(PLANT_MIN_LIFE, PLANT_MAX_LIFE),
                        nutrition: PLANT_INITIAL_NUTRITION,
                    },
                    &mut plants,
                )
                .with(transform, &mut transforms)
                .build();
        }
    }
}
