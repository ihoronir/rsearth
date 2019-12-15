use amethyst::{
    core::timing::Time,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, Read,/* ReadStorage,*/ System, SystemData, World, Entities, WriteStorage}
};
use crate::earth::{
    Herbivore,
    HERBIVORE_BOID_SEPARATION,
    HERBIVORE_BOID_COHERENCE,
    HERBIVORE_BOID_ALIGNMENT,
    HERBIVORE_BOID_SEPARATION_DISTANCE,
    HERBIVORE_BOID_VISIBILITY_LENGTH,
    HERBIVORE_BOID_MAX_SPEED
};

#[derive(SystemDesc)]
pub struct HerbivoreSystem;

impl<'s> System<'s> for HerbivoreSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Herbivore>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut entities, mut herbivores, mut transforms, time): Self::SystemData) {

        // x, y, vx, vy
        let mut herbivores_cache: Vec<(f32, f32, f32, f32)> = vec![];

        for (herbivore, transform) in (&mut herbivores, &mut transforms).join() {
            herbivores_cache.push((transform.translation().x, transform.translation().y, herbivore.vx, herbivore.vy));
        }

        for (herbivore, transform) in (&mut herbivores, &mut transforms).join() {
            let self_x = transform.translation().x;
            let self_y = transform.translation().y;

            let separation = {
                let mut fx = 0.0;
                let mut fy = 0.0;
                for (other_x, other_y, _, _) in &herbivores_cache {
                    let distance_square = (self_x - other_x) * (self_x - other_x) + (self_y - other_y) * (self_y - other_y);
                    if distance_square < HERBIVORE_BOID_SEPARATION_DISTANCE * HERBIVORE_BOID_SEPARATION_DISTANCE && distance_square != 0.0 {
                        // distance ではなく distance_square で割っているのは、
                        // 近くにいるもの同士の反発を強めるため。
                        // 平方根計算コスト削減にもなる。
                        fx -= (other_x - self_x) / distance_square;
                        fy -= (other_y - self_y) / distance_square;
                    }
                }
                (fx, fy)
            };


            let coherence = {
                let mut x_sum = 0.0;
                let mut y_sum = 0.0;
                let mut other_num = 0;
                for (other_x, other_y, _, _) in &herbivores_cache {
                    let distance_square = (self_x - other_x) * (self_x - other_x) + (self_y - other_y) * (self_y - other_y);
                    // ここで自分を除外しないことで、ゼロ除算を回避。
                    // 全体の結果にも影響しない。
                    if distance_square < HERBIVORE_BOID_VISIBILITY_LENGTH * HERBIVORE_BOID_VISIBILITY_LENGTH {
                        x_sum += other_x;
                        y_sum += other_y;
                        other_num += 1;
                    }
                }
                (x_sum / other_num as f32 - self_x, y_sum / other_num as f32 - self_y)
            };

            let alignment = {
                let mut vx_sum = 0.0;
                let mut vy_sum = 0.0;
                let mut other_num = 0;
                for (other_x, other_y, other_vx, other_vy) in &herbivores_cache {
                    let distance_square = (self_x - other_x) * (self_x - other_x) + (self_y - other_y) * (self_y - other_y);
                    if distance_square < HERBIVORE_BOID_VISIBILITY_LENGTH * HERBIVORE_BOID_VISIBILITY_LENGTH && distance_square != 0.0 {
                        vx_sum += other_vx;
                        vy_sum += other_vy;
                        other_num += 1;
                    }
                }
                if other_num != 0 {
                    (vx_sum / other_num as f32, vy_sum / other_num as f32)
                } else {
                    (0.0, 0.0)
                }
            };

            let ax = separation.0 * HERBIVORE_BOID_SEPARATION
                   + coherence.0  * HERBIVORE_BOID_COHERENCE
                   + alignment.0  * HERBIVORE_BOID_ALIGNMENT;

            let ay = separation.1 * HERBIVORE_BOID_SEPARATION
                   + coherence.1  * HERBIVORE_BOID_COHERENCE
                   + alignment.1  * HERBIVORE_BOID_ALIGNMENT;

            herbivore.vx += ax;
            herbivore.vy += ay;

            // 最高スピード以下に抑える
            let v_norm = (herbivore.vx * herbivore.vx + herbivore.vy * herbivore.vy).sqrt();
            if HERBIVORE_BOID_MAX_SPEED < v_norm {
                herbivore.vx = herbivore.vx * HERBIVORE_BOID_MAX_SPEED / v_norm;
                herbivore.vy = herbivore.vy * HERBIVORE_BOID_MAX_SPEED / v_norm;
            }

            transform.prepend_translation_x(herbivore.vx * time.delta_seconds());
            transform.prepend_translation_y(herbivore.vy * time.delta_seconds());
        }

        for (entity, herbivore) in (&*entities, &mut herbivores).join() {
        }
    }
}

