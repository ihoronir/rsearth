use crate::earth::{
    Acceleration, Plant, Velocity, PLANT_FRICTION, PLANT_SEPARATION, PLANT_SEPARATION_DISTANCE,
};
use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, ParJoin, ReadStorage, System, SystemData, World, WriteStorage},
};
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

    fn run(&mut self, (transforms, velocities, mut accelerations, plants): Self::SystemData) {
        (&plants, &transforms, &mut accelerations, &velocities)
            .par_join()
            .for_each(|(_, self_transform, acceleration, velocity)| {
                let self_x = self_transform.translation().x;
                let self_y = self_transform.translation().y;

                let separation = {
                    let mut fx = 0.0;
                    let mut fy = 0.0;

                    for (_, other_transform) in (&plants, &transforms).join() {
                        let other_x = other_transform.translation().x;
                        let other_y = other_transform.translation().y;
                        let distance_square = (self_x - other_x) * (self_x - other_x)
                            + (self_y - other_y) * (self_y - other_y);
                        if distance_square < PLANT_SEPARATION_DISTANCE * PLANT_SEPARATION_DISTANCE
                            && distance_square != 0.0
                        {
                            fx -= (other_x - self_x) / distance_square;
                            fy -= (other_y - self_y) / distance_square;
                        }
                    }
                    (fx, fy)
                };

                let mut new_ax = separation.0 * PLANT_SEPARATION;
                let mut new_ay = separation.1 * PLANT_SEPARATION;

                if velocity.x != 0.0 || velocity.y != 0.0 {
                    new_ax -= velocity.x * PLANT_FRICTION;
                    new_ay -= velocity.y * PLANT_FRICTION;
                }

                acceleration.x = new_ax;
                acceleration.y = new_ay;
            });
    }
}
