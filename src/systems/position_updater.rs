use crate::earth::{Acceleration, Velocity, GROUND_HEIGHT, GROUND_WIDTH};
use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct PositionUpdater;

impl<'s> System<'s> for PositionUpdater {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Acceleration>,
    );

    fn run(&mut self, (mut transforms, mut velocities, accelerations): Self::SystemData) {
        for (transform, velocity, acceleration) in
            (&mut transforms, &mut velocities, &accelerations).join()
        {
            let mut new_vx = velocity.x + acceleration.x;
            let mut new_vy = velocity.y + acceleration.y;

            let new_speed = (new_vx * new_vx + new_vy * new_vy).sqrt();
            if acceleration.max_speed < new_speed {
                new_vx = new_vx * acceleration.max_speed / new_speed;
                new_vy = new_vy * acceleration.max_speed / new_speed;
            }

            let new_x = transform.translation().x + new_vx * 1.0 / 60.0;
            let new_y = transform.translation().y + new_vy * 1.0 / 60.0;

            velocity.x = new_vx;
            velocity.y = new_vy;

            if new_x < 0.0 {
                transform.set_translation_x(GROUND_WIDTH + new_x);
            } else if GROUND_WIDTH < new_x {
                transform.set_translation_x(new_x - GROUND_WIDTH);
            } else {
                transform.set_translation_x(new_x);
            }

            if new_y < 0.0 {
                transform.set_translation_y(GROUND_HEIGHT + new_y);
            } else if GROUND_HEIGHT < new_y {
                transform.set_translation_y(new_y - GROUND_HEIGHT);
            } else {
                transform.set_translation_y(new_y);
            }
        }
    }
}
