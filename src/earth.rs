extern crate amethyst;

use rand::Rng;
use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{transform::Transform},
    prelude::*,
    ecs::prelude::{Component, VecStorage},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const GROUND_HEIGHT: f32 = 1080.0;
pub const GROUND_WIDTH: f32 = 1920.0;

// Earth

#[derive(Default)]
pub struct Earth;

impl SimpleState for Earth {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let creature_sprite_sheet_handle = load_creature_sprite_sheet(world);

        world.register::<Plant>();
        world.register::<Herbivore>();
        world.register::<Carnivore>();

        initialise_creatures(world, creature_sprite_sheet_handle);
        initialise_camera(world);
    }
}

// Velocity

#[derive(Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

// Acceleration

#[derive(Default)]
pub struct Acceleration {
    pub max_speed: f32,
    pub x: f32,
    pub y: f32
}

impl Component for Acceleration {
    type Storage = VecStorage<Self>;
}

// Plant
// 毎フレームごとにランダムに NUTRITION 増加
// 自身の NUTRITION が INITIAL_NUTRITION * 2 になったら種を撒く。
// 種を撒いたら INITIAL_NUTRITION 減る。

pub const PLANT_MIN_LIFE: u32 = 100;        // 寿命の下限値
pub const PLANT_MAX_LIFE: u32 = 300;        // 寿命の上限値
pub const PLANT_INITIAL_NUTRITION: u32 = 400;
pub const PLANT_MAX_SPEED: f32 = 120.0;
pub const PLANT_BOID_FRICTION: f32 = 0.1;
pub const PLANT_BOID_SEPARATION_DISTANCE: f32 = 1.0;
pub const PLANT_BOID_SEPARATION: f32 = 200.0;

#[derive(Default)]
pub struct Plant {
   pub life: u32,
   pub nutrition: u32
}

impl Component for Plant {
    type Storage = VecStorage<Self>;
}

// Herbivoreo
// Plant を食べることで Plant が持っていた NUTRITION を吸収
// 自身の NUTRITION が INITIAL_NUTRITION * 2 になったら子供を産む。
// 子供を産んだら INITIAL_NUTRITION 減る。

pub const HERBIVORE_MIN_LIFE: u32 = 100;
pub const HERBIVORE_MAX_LIFE: u32 = 400;
pub const HERBIVORE_INITIAL_NUTRITION: u32 = 3600;
pub const HERBIVORE_REACHABLE_RANGE: f32 = 4.0;
pub const HERBIVORE_MAX_SPEED: f32 = 30.0;          // 最高速度
pub const HERBIVORE_BOID_SEPARATION_DISTANCE: f32 = 50.0; // 最適な間隔
pub const HERBIVORE_BOID_SEPARATION: f32 = 200.0;         // 間隔をとろうとする度合い
pub const HERBIVORE_BOID_COHERENCE: f32 = 0.85;            // 群れの中心に向かう度合い
pub const HERBIVORE_BOID_ALIGNMENT: f32 = 0.02;           // 整列しようとする度合い
pub const HERBIVORE_BOID_GRAVITY: f32 = 0.9;              // 餌に引き着く度合い
pub const HERBIVORE_BOID_VISIBILITY_LENGTH: f32 = 80.0;   // 見えている長さ

#[derive(Default)]
pub struct Herbivore {
    pub life: u32,
    pub nutrition: u32,
}

impl Component for Herbivore {
    type Storage = VecStorage<Self>;
}

// Carnivore

#[derive(Default)]
pub struct Carnivore;

impl Component for Carnivore {
    type Storage = VecStorage<Self>;
}

// Initialises

fn initialise_creatures(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    let mut rng = rand::thread_rng();

    {
        // Plants

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 2
        };

        for _ in 0..500 {

            let mut transform = Transform::default();
            transform.set_translation_xyz(rng.gen_range(0.0, GROUND_WIDTH), rng.gen_range(0.0, GROUND_HEIGHT), 0.0);

            world
                .create_entity()
                .with(sprite_render.clone())
                .with(Velocity{x: 0.0, y: 0.0})
                .with(Acceleration{max_speed: PLANT_MAX_SPEED, x: 0.0, y: 0.0})
                .with(
                    Plant{
                        life: rng.gen_range(PLANT_MIN_LIFE, PLANT_MAX_LIFE),
                        nutrition: PLANT_INITIAL_NUTRITION
                    }
                )
                .with(transform)
                .build();
        }
    }

    {
        // Herbivore

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 1
        };

        for _ in 0..200 {

            let mut transform = Transform::default();
            transform.set_translation_xyz(rng.gen_range(0.0, GROUND_WIDTH), rng.gen_range(0.0, GROUND_HEIGHT), 0.0);

            world
                .create_entity()
                .with(sprite_render.clone())
                .with(Velocity{x: 0.0, y: 0.0})
                .with(Acceleration{max_speed: HERBIVORE_MAX_SPEED, x: 0.0, y: 0.0})
                .with(
                    Herbivore{
                        life: rng.gen_range(HERBIVORE_MIN_LIFE, HERBIVORE_MAX_LIFE),
                        nutrition: HERBIVORE_INITIAL_NUTRITION,
                    }
                )
                .with(transform)
                .build();
        }
    }

}

// fn incubate_plant(sprite_render, life, nutrition, transform) 

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(GROUND_WIDTH * 0.5, GROUND_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(GROUND_WIDTH, GROUND_HEIGHT))
        .with(transform)
        .build();
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
