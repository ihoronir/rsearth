extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{transform::Transform},
    prelude::*,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

const GROUND_HEIGHT: f32 = 600.0;
const GROUND_WIDTH: f32 = 600.0;

// Earth

#[derive(Default)]
pub struct Earth;

impl SimpleState for Earth {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let creature_sprite_sheet_handle = load_creature_sprite_sheet(world);

        world.register::<CreatureA>();

        initialise_creatures(world, creature_sprite_sheet_handle);
        initialise_camera(world);
    }
}

// Creature A

#[derive(Default)]
struct CreatureA;

impl Component for CreatureA {
    type Storage = DenseVecStorage<Self>;
}

// Creature B

#[derive(Default)]
struct CreatureB;

impl Component for CreatureB {
    type Storage = DenseVecStorage<Self>;
}

// Initialises

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(GROUND_WIDTH * 0.5, GROUND_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(GROUND_WIDTH, GROUND_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_creatures(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    let sprite_render_a = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0
    };

    let sprite_render_b = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1
    };

    for i in 0..20 {
        for j in 0..20 {

            let mut transform = Transform::default();
            transform.set_translation_xyz(GROUND_WIDTH  / 20.0 * i as f32, GROUND_HEIGHT / 20.0 * j as f32, 0.0);

            world
                .create_entity()
                .with(sprite_render_a.clone())
                .with(CreatureA::default())
                .with(transform)
                .build();
        }
    }

}

// Sprite Sheet Loader

fn load_creature_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/creature.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/creature.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}
