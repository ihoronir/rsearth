use amethyst::{
    // core::timing::Time,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, ParJoin,/* Read,*/ ReadStorage, System, SystemData, World, WriteStorage},
    //renderer::SpriteRender
};
use rayon::prelude::*;
use crate::earth::{
    Velocity,
    Acceleration,
    Plant,
    Herbivore,
    HERBIVORE_BOID_SEPARATION,
    HERBIVORE_BOID_COHERENCE,
    HERBIVORE_BOID_ALIGNMENT,
    HERBIVORE_BOID_GRAVITY,
    HERBIVORE_BOID_SEPARATION_DISTANCE,
    HERBIVORE_BOID_VISIBILITY_LENGTH,
};

#[derive(SystemDesc)]
pub struct HerbivoreMechanics;

impl<'s> System<'s> for HerbivoreMechanics {

    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
        ReadStorage<'s, Plant>,
        ReadStorage<'s, Herbivore>,
    );

    fn run(
        &mut self,
        (
            transforms,
            velocities,
            mut accelerations,
            plants,
            herbivores,
        ): Self::SystemData
    ) {
        
        (&herbivores, &transforms, &mut accelerations,/* &velocities*/).par_join()
            .for_each(|(_, self_transform, mut acceleration,/* velocity*/)| {
                let self_x = self_transform.translation().x;
                let self_y = self_transform.translation().y;

                // counter
                let mut visible_num = 0;

                // variables for separation
                let mut fx = 0.0;
                let mut fy = 0.0;

                // variables for coherence
                let mut visible_x_sum = 0.0;
                let mut visible_y_sum = 0.0;

                // variables for alignment
                let mut visible_vx_sum = 0.0;
                let mut visible_vy_sum = 0.0;

                for (_, other_transform, other_velocity) in (&herbivores, &transforms, &velocities).join() {
                    let other_x = other_transform.translation().x;
                    let other_y = other_transform.translation().y;
                    let other_vx = other_velocity.x;
                    let other_vy = other_velocity.y;

                    let distance_square = (self_x - other_x) * (self_x - other_x) + (self_y - other_y) * (self_y - other_y);

                    if distance_square < HERBIVORE_BOID_VISIBILITY_LENGTH * HERBIVORE_BOID_VISIBILITY_LENGTH && distance_square != 0.0 {
                        visible_num += 1;

                        visible_x_sum += other_x;
                        visible_y_sum += other_y;

                        visible_vx_sum += other_vx;
                        visible_vy_sum += other_vy;

                        if distance_square < HERBIVORE_BOID_SEPARATION_DISTANCE * HERBIVORE_BOID_SEPARATION_DISTANCE {
                            fx -= (other_x - self_x) / distance_square;
                            fy -= (other_y - self_y) / distance_square;
                        }
                    }
                }

                let separation = (fx, fy);

                let coherence = {
                    if visible_num == 0 {
                        (0.0, 0.0)
                    } else {
                        (visible_x_sum / visible_num as f32 - self_x, visible_y_sum / visible_num as f32 - self_y)
                    }
                };

                let alignment = {
                    if visible_num == 0 {
                        (0.0, 0.0)
                    } else {
                        (visible_vx_sum / visible_num as f32, visible_vy_sum / visible_num as f32)
                    }
                };

                let plant_gravity = {
                    let mut x_sum = 0.0;
                    let mut y_sum = 0.0;
                    let mut plants_num = 0;
                    for (_, plant_transform) in (&plants, &transforms).join() {
                        let plant_x = plant_transform.translation().x;
                        let plant_y = plant_transform.translation().y;
                        let distance_square = (self_x - plant_x) * (self_x - plant_x) + (self_y - plant_y) * (self_y - plant_y);
                        if distance_square < HERBIVORE_BOID_VISIBILITY_LENGTH * HERBIVORE_BOID_VISIBILITY_LENGTH {
                            x_sum += plant_x;
                            y_sum += plant_y;
                            plants_num += 1;
                        }
                    }
                    if plants_num != 0 {
                        (x_sum / plants_num as f32 - self_x, y_sum / plants_num as f32 - self_y)
                    } else {
                        (0.0, 0.0)
                    }
                };

                let ax = separation.0    * HERBIVORE_BOID_SEPARATION
                       + coherence.0     * HERBIVORE_BOID_COHERENCE
                       + alignment.0     * HERBIVORE_BOID_ALIGNMENT
                       + plant_gravity.0 * HERBIVORE_BOID_GRAVITY;

                let ay = separation.1    * HERBIVORE_BOID_SEPARATION
                       + coherence.1     * HERBIVORE_BOID_COHERENCE
                       + alignment.1     * HERBIVORE_BOID_ALIGNMENT
                       + plant_gravity.1 * HERBIVORE_BOID_GRAVITY;

                acceleration.x = ax;
                acceleration.y = ay;

            });
        }

        // let mut new_herbivores: Vec<(Transform, SpriteRender)> = vec![];

        //for (creature, herbivore, transform, sprite_render) in (&mut creatures, &mut herbivores, &mut transforms, &mut sprite_renders).join() {
        //    // transform.prepend_translation_x(herbivore.vx * time.delta_seconds());
        //    // transform.prepend_translation_y(herbivore.vy * time.delta_seconds());
        //    transform.prepend_translation_x(herbivore.vx * 1.0 / 60.0);
        //    transform.prepend_translation_y(herbivore.vy * 1.0 / 60.0);

        //    if creature.nutrition > HERBIVORE_INITIAL_NUTRITION * 2 {
        //        let difference = {
        //            let r = PI * 2.0 * rng.gen::<f32>();
        //            let s = HERBIVORE_BOID_SEPARATION_DISTANCE / 2.0 * (rng.gen::<f32>().sqrt());
        //            (s * r.cos(), s * r.sin())
        //        };
        //        let mut new_transform = Transform::default();
        //        new_transform.set_translation_xyz(transform.translation().x + difference.0 , transform.translation().y + difference.1, 0.0);
        //        let new_sprite_render = sprite_render.clone();
        //        new_herbivores.push((new_transform, new_sprite_render));

        //        creature.nutrition -= HERBIVORE_INITIAL_NUTRITION;
        //    } 
        //}

        //for (transform, sprite_render) in new_herbivores {
        //    entities
        //        .build_entity()
        //        .with(sprite_render, &mut sprite_renders)
        //        .with(Creature{life: rng.gen_range(HERBIVORE_MIN_LIFE, HERBIVORE_MAX_LIFE), nutrition: HERBIVORE_INITIAL_NUTRITION}, &mut creatures)
        //        .with(Herbivore::default(), &mut herbivores)
        //        .with(transform, &mut transforms)
        //        .build();
        //}
}
