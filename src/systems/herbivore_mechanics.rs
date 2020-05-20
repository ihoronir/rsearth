use crate::earth::{
    Acceleration, Carnivore, Herbivore, Plant, Velocity, HERBIVORE_ALIGNMENT,
    HERBIVORE_CARNIVORE_GRAVITY, HERBIVORE_COHERENCE, HERBIVORE_PLANT_GRAVITY,
    HERBIVORE_SEPARATION, HERBIVORE_SEPARATION_DISTANCE, HERBIVORE_VISIBLE_DISTANCE,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ParJoin, ReadStorage, System, SystemData, WriteStorage},
};
use rayon::prelude::*;

#[derive(SystemDesc)]
pub struct HerbivoreMechanics;

impl<'s> System<'s> for HerbivoreMechanics {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Acceleration>,
        ReadStorage<'s, Plant>,
        ReadStorage<'s, Carnivore>,
        ReadStorage<'s, Herbivore>,
    );

    fn run(
        &mut self,
        (
            transforms,
            velocities,
            mut accelerations,
            plants,
            carnivores,
            herbivores,
        ): Self::SystemData,
    ) {
        (&herbivores, &transforms, &mut accelerations)
            .par_join()
            .for_each(|(_, self_transform, mut acceleration)| {
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

                for (_, other_transform, other_velocity) in
                    (&herbivores, &transforms, &velocities).join()
                {
                    let other_x = other_transform.translation().x;
                    let other_y = other_transform.translation().y;
                    let other_vx = other_velocity.x;
                    let other_vy = other_velocity.y;

                    let distance_square = (self_x - other_x) * (self_x - other_x)
                        + (self_y - other_y) * (self_y - other_y);

                    if distance_square < HERBIVORE_VISIBLE_DISTANCE * HERBIVORE_VISIBLE_DISTANCE
                        && distance_square != 0.0
                    {
                        visible_num += 1;

                        visible_x_sum += other_x;
                        visible_y_sum += other_y;

                        visible_vx_sum += other_vx;
                        visible_vy_sum += other_vy;

                        if distance_square
                            < HERBIVORE_SEPARATION_DISTANCE * HERBIVORE_SEPARATION_DISTANCE
                        {
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
                        (
                            visible_x_sum / visible_num as f32 - self_x,
                            visible_y_sum / visible_num as f32 - self_y,
                        )
                    }
                };

                let alignment = {
                    if visible_num == 0 {
                        (0.0, 0.0)
                    } else {
                        (
                            visible_vx_sum / visible_num as f32,
                            visible_vy_sum / visible_num as f32,
                        )
                    }
                };

                let plant_gravity = {
                    let mut x_sum = 0.0;
                    let mut y_sum = 0.0;
                    let mut plants_num = 0;
                    for (_, plant_transform) in (&plants, &transforms).join() {
                        let plant_x = plant_transform.translation().x;
                        let plant_y = plant_transform.translation().y;
                        let distance_square = (self_x - plant_x) * (self_x - plant_x)
                            + (self_y - plant_y) * (self_y - plant_y);
                        if distance_square < HERBIVORE_VISIBLE_DISTANCE * HERBIVORE_VISIBLE_DISTANCE
                        {
                            x_sum += plant_x;
                            y_sum += plant_y;
                            plants_num += 1;
                        }
                    }
                    if plants_num != 0 {
                        (
                            x_sum / plants_num as f32 - self_x,
                            y_sum / plants_num as f32 - self_y,
                        )
                    } else {
                        (0.0, 0.0)
                    }
                };

                let carnivore_gravity = {
                    let mut fx = 0.0;
                    let mut fy = 0.0;
                    for (_, carnivore_transform) in (&carnivores, &transforms).join() {
                        let carnivore_x = carnivore_transform.translation().x;
                        let carnivore_y = carnivore_transform.translation().y;
                        let distance_square = (self_x - carnivore_x) * (self_x - carnivore_x)
                            + (self_y - carnivore_y) * (self_y - carnivore_y);
                        if distance_square < HERBIVORE_VISIBLE_DISTANCE * HERBIVORE_VISIBLE_DISTANCE
                            && distance_square != 0.0
                        {
                            fx -= (carnivore_x - self_x) / distance_square;
                            fy -= (carnivore_y - self_y) / distance_square;
                        }
                    }
                    (fx, fy)
                };

                let ax = separation.0 * HERBIVORE_SEPARATION
                    + coherence.0 * HERBIVORE_COHERENCE
                    + alignment.0 * HERBIVORE_ALIGNMENT
                    + plant_gravity.0 * HERBIVORE_PLANT_GRAVITY
                    + carnivore_gravity.0 * HERBIVORE_CARNIVORE_GRAVITY;

                let ay = separation.1 * HERBIVORE_SEPARATION
                    + coherence.1 * HERBIVORE_COHERENCE
                    + alignment.1 * HERBIVORE_ALIGNMENT
                    + plant_gravity.1 * HERBIVORE_PLANT_GRAVITY
                    + carnivore_gravity.1 * HERBIVORE_CARNIVORE_GRAVITY;

                acceleration.x = ax;
                acceleration.y = ay;
            });
    }
}
