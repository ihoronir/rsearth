//use std::f32::consts::PI;
//use rand::Rng;
use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, ParJoin, /* Read,*/ ReadStorage, System, SystemData, World/*, Entities*/, WriteStorage},
    //renderer::SpriteRender
};
use crate::earth::{Velocity, Acceleration, Plant, PLANT_BOID_SEPARATION_DISTANCE, PLANT_BOID_SEPARATION, PLANT_BOID_FRICTION};
use rayon::prelude::*;

#[derive(SystemDesc)]
pub struct PlantMechanics;

impl<'s> System<'s> for PlantMechanics {

    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
        ReadStorage<'s, Plant>,
    );

    fn run(
        &mut self,
        (
            transforms,
            velocities,
            mut accelerations,
            plants,
        ): Self::SystemData
    ) {
        (&plants, &transforms, &mut accelerations, &velocities).par_join()
            .for_each(|(_, self_transform, acceleration, velocity)| {
                let self_x = self_transform.translation().x;
                let self_y = self_transform.translation().y;

                let coherence = {
                    let mut fx = 0.0;
                    let mut fy = 0.0;

                    for (_, other_transform) in (&plants, &transforms).join() {
                        let other_x = other_transform.translation().x;
                        let other_y = other_transform.translation().y;
                        let distance_square = (self_x - other_x) * (self_x - other_x) + (self_y - other_y) * (self_y - other_y);
                        if distance_square < PLANT_BOID_SEPARATION_DISTANCE * PLANT_BOID_SEPARATION_DISTANCE && distance_square != 0.0 {
                            fx -= (other_x - self_x) / distance_square;
                            fy -= (other_y - self_y) / distance_square;
                        }
                    }
                    (fx, fy)
                };

                let mut new_ax = coherence.0 * PLANT_BOID_SEPARATION;
                let mut new_ay = coherence.1 * PLANT_BOID_SEPARATION;

                let speed_square = velocity.x * velocity.x + velocity.y * velocity.y;
                if speed_square != 0.0 {
                    new_ax -= velocity.x * PLANT_BOID_FRICTION;
                    new_ay -= velocity.y * PLANT_BOID_FRICTION;
                }

                acceleration.x = new_ax;
                acceleration.y = new_ay;
            });
    }
}