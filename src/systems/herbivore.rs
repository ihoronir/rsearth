use std::f32::consts::PI;
use rand::Rng;
use amethyst::{
    core::timing::Time,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, Entities, WriteStorage},
    renderer::SpriteRender
};
use crate::earth::{
    Creature,
    Plant,
    Herbivore,
    HERBIVORE_MAX_LIFE,
    HERBIVORE_MIN_LIFE,
    HERBIVORE_INITIAL_NUTRITION,
    HERBIVORE_BOID_SEPARATION,
    HERBIVORE_BOID_COHERENCE,
    HERBIVORE_BOID_ALIGNMENT,
    HERBIVORE_BOID_GRAVITY,
    HERBIVORE_BOID_SEPARATION_DISTANCE,
    HERBIVORE_BOID_VISIBILITY_LENGTH,
    HERBIVORE_BOID_MAX_SPEED
};

#[derive(SystemDesc)]
pub struct HerbivoreSystem;

impl<'s> System<'s> for HerbivoreSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Creature>,
        ReadStorage<'s, Plant>,
        WriteStorage<'s, Herbivore>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>
    );

    fn run(
        &mut self,
        (
            entities,
            mut creatures,
            plants,
            mut herbivores,
            mut transforms,
            mut sprite_renders,
            time
        ): Self::SystemData
    ) {
        let mut rng = rand::thread_rng();

        // x, y, vx, vy
        let mut herbivores_cache: Vec<(f32, f32, f32, f32)> = vec![];

        for (herbivore, transform) in (&herbivores, &transforms).join() {
            herbivores_cache.push((transform.translation().x, transform.translation().y, herbivore.vx, herbivore.vy));
        }

        // x, y
        let mut plants_cache: Vec<(f32, f32)> = vec![];

        for (_, transform) in (&plants, &transforms).join() {
            plants_cache.push((transform.translation().x, transform.translation().y));
        }

        for (herbivore, transform) in (&mut herbivores, &mut transforms).join() {
            let self_x = transform.translation().x;
            let self_y = transform.translation().y;

            // x, y, vx, vy
            let (visible_others, near_others) = {
                // x, y, vx, vy
                let mut visible: Vec<&(f32, f32, f32, f32)> = vec![];
                // ((x, y, vx, vy), distance_square)
                let mut near: Vec<(&(f32, f32, f32, f32), f32)> = vec![];
                for other in &herbivores_cache {
                    let (other_x, other_y, _, _) = other;
                    let distance_square = (self_x - other_x) * (self_x - other_x) + (self_y - other_y) * (self_y - other_y);
                    if distance_square < HERBIVORE_BOID_VISIBILITY_LENGTH * HERBIVORE_BOID_VISIBILITY_LENGTH && distance_square != 0.0 {
                        visible.push(&other);
                        if distance_square < HERBIVORE_BOID_SEPARATION_DISTANCE * HERBIVORE_BOID_SEPARATION_DISTANCE {
                            near.push((&other, distance_square));
                        }
                    }
                }
                (visible, near)
            };

            let separation = {
                let mut fx = 0.0;
                let mut fy = 0.0;
                for ((other_x, other_y, _, _), distance_square) in &near_others {
                    // distance ではなく distance_square で割っているのは、
                    // 近くにいるもの同士の反発を強めるため。
                    // 平方根計算コスト削減にもなる。
                    fx -= (other_x - self_x) / distance_square;
                    fy -= (other_y - self_y) / distance_square;
                }
                (fx, fy)
            };


            let coherence = {
                if visible_others.len() != 0 {
                    let mut x_sum = 0.0;
                    let mut y_sum = 0.0;
                    for (other_x, other_y, _, _) in &visible_others {
                        x_sum += other_x;
                        y_sum += other_y;
                    }
                    (x_sum / visible_others.len() as f32 - self_x, y_sum / visible_others.len() as f32 - self_y)
                } else {
                    (0.0, 0.0)
                }
            };

            let alignment = {
                if visible_others.len() != 0 {
                    let mut vx_sum = 0.0;
                    let mut vy_sum = 0.0;
                    for (_, _, other_vx, other_vy) in &visible_others {
                        vx_sum += other_vx;
                        vy_sum += other_vy;
                    }
                    (vx_sum / visible_others.len() as f32, vy_sum / visible_others.len() as f32)
                } else {
                    (0.0, 0.0)
                }
            };

            let plant_gravity = {
                let mut x_sum = 0.0;
                let mut y_sum = 0.0;
                let mut plants_num = 0;
                for (plant_x, plant_y) in &plants_cache {
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

            herbivore.vx += ax;
            herbivore.vy += ay;

            // 最高スピード以下に抑える
            let v_norm = (herbivore.vx * herbivore.vx + herbivore.vy * herbivore.vy).sqrt();
            if HERBIVORE_BOID_MAX_SPEED < v_norm {
                herbivore.vx = herbivore.vx * HERBIVORE_BOID_MAX_SPEED / v_norm;
                herbivore.vy = herbivore.vy * HERBIVORE_BOID_MAX_SPEED / v_norm;
            }
        }

        let mut new_herbivores: Vec<(Transform, SpriteRender)> = vec![];

        for (creature, herbivore, transform, sprite_render) in (&mut creatures, &mut herbivores, &mut transforms, &mut sprite_renders).join() {
            // transform.prepend_translation_x(herbivore.vx * time.delta_seconds());
            // transform.prepend_translation_y(herbivore.vy * time.delta_seconds());

            transform.prepend_translation_x(herbivore.vx * 1.0 / 60.0);
            transform.prepend_translation_y(herbivore.vy * 1.0 / 60.0);

            if creature.nutrition > HERBIVORE_INITIAL_NUTRITION * 2 {
                let difference = {
                    let r = PI * 2.0 * rng.gen::<f32>();
                    let s = HERBIVORE_BOID_SEPARATION_DISTANCE / 2.0 * (rng.gen::<f32>().sqrt());
                    (s * r.cos(), s * r.sin())
                };
                let mut new_transform = Transform::default();
                new_transform.set_translation_xyz(transform.translation().x + difference.0 , transform.translation().y + difference.1, 0.0);
                let new_sprite_render = sprite_render.clone();
                new_herbivores.push((new_transform, new_sprite_render));

                creature.nutrition -= HERBIVORE_INITIAL_NUTRITION;
            } 
        }

        for (transform, sprite_render) in new_herbivores {
            entities
                .build_entity()
                .with(sprite_render, &mut sprite_renders)
                .with(Creature{life: rng.gen_range(HERBIVORE_MIN_LIFE, HERBIVORE_MAX_LIFE), nutrition: HERBIVORE_INITIAL_NUTRITION}, &mut creatures)
                .with(Herbivore::default(), &mut herbivores)
                .with(transform, &mut transforms)
                .build();
        }
    }
}

