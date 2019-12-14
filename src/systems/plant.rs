use std::f32::consts::PI;
use rand::Rng;
use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, /*Read, ReadStorage,*/ System, SystemData, World, Entities, WriteStorage},
    renderer::SpriteRender
};
use crate::earth::{Creature, Plant, PLANT_MIN_LIFE, PLANT_MAX_LIFE, PLANT_INITIAL_NUTRITION};


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

        for (creature, _, transform, sprite_render) in (&mut creatures, &mut plants, &mut transforms, &mut sprite_renders).join() {

            if creature.nutrition > PLANT_INITIAL_NUTRITION * 2 {
                let difference = {
                    let r = PI * 2.0 * rng.gen::<f32>();
                    let s = 40.0 * (rng.gen::<f32>().sqrt());
                    (s * r.cos(), s * r.sin())
                };
                let mut new_transform = Transform::default();
                new_transform.set_translation_xyz(transform.translation().x + difference.0 , transform.translation().y + difference.1, 0.0);
                let new_sprite_render = sprite_render.clone();
                new_plants.push((new_transform, new_sprite_render));

                creature.nutrition -= PLANT_INITIAL_NUTRITION;

            } 
        }

        for (transform, sprite_render) in new_plants {
            entities
                .build_entity()
                .with(sprite_render, &mut sprite_renders)
                .with(Creature{life: rng.gen_range(PLANT_MIN_LIFE, PLANT_MAX_LIFE), nutrition: PLANT_INITIAL_NUTRITION}, &mut creatures)
                .with(Plant::default(), &mut plants)
                .with(transform, &mut transforms)
                .build();
        }
    }
}
