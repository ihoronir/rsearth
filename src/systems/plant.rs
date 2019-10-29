use std::f32::consts::PI;
use rand::Rng;
use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, /*Read, ReadStorage,*/ System, SystemData, World, Entities, WriteStorage},
    renderer::SpriteRender
};
use crate::earth::{Creature, Plant};


#[derive(SystemDesc)]
pub struct PlantSystem;

impl<'s> System<'s> for PlantSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Creature>,
        WriteStorage<'s, Plant>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>
    );

    fn run(
        &mut self,
        (
            entities,
            mut creatures,
            mut plants,
            mut transforms,
            mut sprite_renders
        ): Self::SystemData
    ) {
        let mut rng = rand::thread_rng();
        let mut new_plants: Vec<(Transform, SpriteRender)> = vec![];
        for (plant, transform, sprite_render) in (&mut plants, &mut transforms, &mut sprite_renders).join() {
            if plant.drop_seed_count == 0 {
                let difference = {
                    let r = PI * 2.0 * rng.gen::<f32>();
                    let s = 40.0 * (rng.gen::<f32>().sqrt());
                    (s * r.cos(), s * r.sin())
                };
                let mut new_transform = Transform::default();
                new_transform.set_translation_xyz(transform.translation().x + difference.0 , transform.translation().y + difference.1, 0.0);
                let new_sprite_render = sprite_render.clone();
                new_plants.push((new_transform, new_sprite_render));

                plant.drop_seed_count = rng.gen_range(100, 120);
            } else {
                plant.drop_seed_count -= 1;
            }
        }
        for (transform, sprite_render) in new_plants {
            entities
                .build_entity()
                .with(sprite_render, &mut sprite_renders)
                .with(Creature{life: 1000}, &mut creatures)
                .with(Plant{drop_seed_count: 200}, &mut plants)
                .with(transform, &mut transforms)
                .build();
        }
    }
}
