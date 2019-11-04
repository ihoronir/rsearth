use amethyst::{
    core::timing::Time,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, Read,/* ReadStorage,*/ System, SystemData, World, Entities, WriteStorage}
};
use crate::earth::{Herbivore};

#[derive(SystemDesc)]
pub struct HerbivoreSystem;

impl<'s> System<'s> for HerbivoreSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Herbivore>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut entities, mut herbivores, mut transform, mut time): Self::SystemData) {

        {
            let mut herbivores_cash: Vec<(f32, f32)> = vec![];

            for (_, transform) in (&mut herbivores, &mut transform).join() {
                herbivores_cash.push((transform.translation().x, transform.translation().y));
            }

            for (_, transform) in (&mut herbivores, &mut transform).join() {
                let self_x = transform.translation().x;
                let self_y = transform.translation().y;
                let separation = {
                    let mut vx = 0.0;
                    let mut vy = 0.0;
                    for (other_x, other_y) in &herbivores_cash {
                        if (self_x - other_x) * (self_x - other_x) + (self_y - other_y) * (self_y - other_y) < 40.0 * 40.0
                        {
                            vx -= other_x - self_x;
                            vy -= other_y - self_y;
                        }
                    }
                    (vx, vy)
                    //let mut x_sum = 0.0;
                    //let mut y_sum = 0.0;
                    //let mut other_num = 0.0;
                    //for (other_x, other_y) in &herbivores_cash {
                    //    let x_diff = other_x - self_x;
                    //    let y_diff = other_y - self_y;
                    //    if x_diff * x_diff + y_diff * y_diff < 40.0 * 40.0 {
                    //        x_sum += other_x;
                    //        y_sum += other_y;
                    //        other_num += 1.0;
                    //    }
                    //}
                    //(x_sum / other_num, y_sum / other_num)
                };
                transform.prepend_translation_x(separation.0 * time.delta_seconds());
                transform.prepend_translation_y(separation.1 * time.delta_seconds());
            }
        }

        for (entity, herbivore) in (&*entities, &mut herbivores).join() {
        }
    }
}

