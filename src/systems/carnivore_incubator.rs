use crate::earth::{
    Acceleration, Carnivore, Velocity, CARNIVORE_INITIAL_NUTRITION, CARNIVORE_MAX_LIFE,
    CARNIVORE_MAX_SPEED, CARNIVORE_MIN_LIFE, CARNIVORE_SEPARATION_DISTANCE
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
pub struct CarnivoreIncubator;

impl<'s> System<'s> for CarnivoreIncubator {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
        WriteStorage<'s, Carnivore>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut velocities,
            mut accelerations,
            mut carnivores,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();
        let mut new_plants: Vec<(Transform, SpriteRender)> = vec![];

        for (entity, carnivore, transform, sprite_render) in
            (&*entities, &mut carnivores, &transforms, &sprite_renders).join()
        {
            if carnivore.life == 0 {
                entities
                    .delete(entity)
                    .expect("Failed to delete a carnivore.");
            } else {
                carnivore.life -= 1;

                if carnivore.nutrition >= CARNIVORE_INITIAL_NUTRITION * 2 {
                    let difference = {
                        let r = PI * 2.0 * rng.gen::<f32>();
                        let s = CARNIVORE_SEPARATION_DISTANCE * (rng.gen::<f32>().sqrt());
                        (s * r.cos(), s * r.sin())
                    };
                    let mut new_transform = Transform::default();
                    new_transform.set_translation_xyz(
                        transform.translation().x + difference.0,
                        transform.translation().y + difference.1,
                        0.0,
                    );
                    let new_sprite_render = sprite_render.clone();
                    new_plants.push((new_transform, new_sprite_render));

                    carnivore.nutrition -= CARNIVORE_INITIAL_NUTRITION;
                }
            }
        }

        for (transform, sprite_render) in new_plants {
            entities
                .build_entity()
                .with(sprite_render, &mut sprite_renders)
                .with(Velocity { x: 0.0, y: 0.0 }, &mut velocities)
                .with(
                    Acceleration {
                        max_speed: CARNIVORE_MAX_SPEED,
                        x: 0.0,
                        y: 0.0,
                    },
                    &mut accelerations,
                )
                .with(
                    Carnivore {
                        life: rng.gen_range(CARNIVORE_MIN_LIFE, CARNIVORE_MAX_LIFE),
                        nutrition: CARNIVORE_INITIAL_NUTRITION,
                    },
                    &mut carnivores,
                )
                .with(transform, &mut transforms)
                .build();
        }
    }
}
