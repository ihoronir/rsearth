use crate::earth::{
    Acceleration, Herbivore, Velocity, HERBIVORE_INITIAL_NUTRITION, HERBIVORE_MAX_LIFE,
    HERBIVORE_MAX_SPEED, HERBIVORE_MIN_LIFE, HERBIVORE_SEPARATION_DISTANCE,
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
pub struct HerbivoreIncubator;

impl<'s> System<'s> for HerbivoreIncubator {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
        WriteStorage<'s, Herbivore>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut velocities,
            mut accelerations,
            mut herbivores,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();
        let mut new_plants: Vec<(Transform, SpriteRender)> = vec![];

        let mut herbivore_num = 0;

        for (entity, herbivore, transform, sprite_render) in
            (&*entities, &mut herbivores, &transforms, &sprite_renders).join()
        {
            herbivore_num += 1;
            if herbivore.life == 0 {
                entities
                    .delete(entity)
                    .expect("Failed to delete a herbivore.");
            } else {
                herbivore.life -= 1;

                if herbivore.nutrition >= HERBIVORE_INITIAL_NUTRITION * 2 {
                    let difference = {
                        let r = PI * 2.0 * rng.gen::<f32>();
                        let s = HERBIVORE_SEPARATION_DISTANCE * (rng.gen::<f32>().sqrt());
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

                    herbivore.nutrition -= HERBIVORE_INITIAL_NUTRITION;
                }
            }
        }
        print!("{} ", herbivore_num);

        for (transform, sprite_render) in new_plants {
            entities
                .build_entity()
                .with(sprite_render, &mut sprite_renders)
                .with(Velocity { x: 0.0, y: 0.0 }, &mut velocities)
                .with(
                    Acceleration {
                        max_speed: HERBIVORE_MAX_SPEED,
                        x: 0.0,
                        y: 0.0,
                    },
                    &mut accelerations,
                )
                .with(
                    Herbivore {
                        life: rng.gen_range(HERBIVORE_MIN_LIFE, HERBIVORE_MAX_LIFE),
                        nutrition: HERBIVORE_INITIAL_NUTRITION,
                    },
                    &mut herbivores,
                )
                .with(transform, &mut transforms)
                .build();
        }
    }
}
