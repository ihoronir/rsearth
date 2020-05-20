use crate::earth::{
    Acceleration, Carnivore, Herbivore, CARNIVORE_HERBIVORE_GRAVITY, CARNIVORE_VISIBLE_DISTANCE,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ParJoin, ReadStorage, System, SystemData, WriteStorage},
};
use rayon::prelude::*;

#[derive(SystemDesc)]
pub struct CarnivoreMechanics;

impl<'s> System<'s> for CarnivoreMechanics {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Acceleration>,
        ReadStorage<'s, Herbivore>,
        ReadStorage<'s, Carnivore>,
    );

    fn run(&mut self, (transforms, mut accelerations, herbivores, carnivores): Self::SystemData) {
        (&carnivores, &transforms, &mut accelerations)
            .par_join()
            .for_each(|(_, self_transform, mut acceleration)| {
                let self_x = self_transform.translation().x;
                let self_y = self_transform.translation().y;

                let target = {
                    let mut position_op: Option<(f32, f32, f32)> = None;

                    for (_, herbivore_transform) in (&herbivores, &transforms).join() {
                        let herbivore_x = herbivore_transform.translation().x;
                        let herbivore_y = herbivore_transform.translation().y;

                        let distance_square = (self_x - herbivore_x) * (self_x - herbivore_x)
                            + (self_y - herbivore_y) * (self_y - herbivore_y);
                        if distance_square < CARNIVORE_VISIBLE_DISTANCE * CARNIVORE_VISIBLE_DISTANCE
                        {
                            if let Some(position) = position_op {
                                if distance_square < position.2 {
                                    position_op = Some((herbivore_x, herbivore_y, distance_square));
                                }
                            } else {
                                position_op = Some((herbivore_x, herbivore_y, distance_square));
                            }
                        }
                    }
                    position_op
                };

                let herbivore_gravity = if let Some(position) = target {
                    let x_diff = position.0 - self_x;
                    let y_diff = position.1 - self_y;
                    let diff_norm = x_diff * x_diff + y_diff * y_diff;
                    (x_diff / diff_norm, y_diff / diff_norm)
                } else {
                    (0.0, 0.0)
                };

                let ax = herbivore_gravity.0 * CARNIVORE_HERBIVORE_GRAVITY;
                let ay = herbivore_gravity.1 * CARNIVORE_HERBIVORE_GRAVITY;

                acceleration.x = ax;
                acceleration.y = ay;
            });
    }
}
