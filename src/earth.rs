extern crate amethyst;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, VecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use rand::Rng;

// Earth
pub const GROUND_HEIGHT: f32 = 1080.0;
pub const GROUND_WIDTH: f32 = 1920.0;

// Plant
pub const PLANT_MIN_LIFE: u32 = 200;
pub const PLANT_MAX_LIFE: u32 = 300;
pub const PLANT_INITIAL_NUTRITION: u32 = 220;
pub const PLANT_MAX_SPEED: f32 = 120.0;
pub const PLANT_FRICTION: f32 = 0.1;
pub const PLANT_SEPARATION_DISTANCE: f32 = 1.0;
pub const PLANT_SEPARATION: f32 = 200.0;

// Herbivore
pub const HERBIVORE_MIN_LIFE: u32 = 400;
pub const HERBIVORE_MAX_LIFE: u32 = 1000;
pub const HERBIVORE_INITIAL_NUTRITION: u32 = 600;
pub const HERBIVORE_REACHABLE_RANGE: f32 = 6.0;
pub const HERBIVORE_MAX_SPEED: f32 = 80.0;
pub const HERBIVORE_SEPARATION_DISTANCE: f32 = 64.0;
pub const HERBIVORE_SEPARATION: f32 = 200.0;
pub const HERBIVORE_COHERENCE: f32 = 0.65;
pub const HERBIVORE_ALIGNMENT: f32 = 0.02;
pub const HERBIVORE_PLANT_GRAVITY: f32 = 0.9;
pub const HERBIVORE_CARNIVORE_GRAVITY: f32 = 1000.0;
pub const HERBIVORE_VISIBLE_DISTANCE: f32 = 180.0;

// Carnivore
pub const CARNIVORE_MIN_LIFE: u32 = 200;
pub const CARNIVORE_MAX_LIFE: u32 = 600;
pub const CARNIVORE_INITIAL_NUTRITION: u32 = 900;
pub const CARNIVORE_REACHABLE_RANGE: f32 = 4.0;
pub const CARNIVORE_MAX_SPEED: f32 = 160.0;
pub const CARNIVORE_HERBIVORE_GRAVITY: f32 = 300.0;
pub const CARNIVORE_VISIBLE_DISTANCE: f32 = 320.0;

// Velocity

#[derive(Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

// Acceleration

#[derive(Default)]
pub struct Acceleration {
    pub max_speed: f32,
    pub x: f32,
    pub y: f32,
}

impl Component for Acceleration {
    type Storage = VecStorage<Self>;
}

// Plant

#[derive(Default)]
pub struct Plant {
    pub life: u32,
    pub nutrition: u32,
}

impl Component for Plant {
    type Storage = VecStorage<Self>;
}

// Herbivore

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
pub struct Carnivore {
    pub life: u32,
    pub nutrition: u32,
}

impl Component for Carnivore {
    type Storage = VecStorage<Self>;
}

// Earth
#[derive(Default)]
pub struct Earth;

impl SimpleState for Earth {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let creature_sprite_sheet_handle = load_creature_sprite_sheet(world);

        initialise_creatures(world, creature_sprite_sheet_handle);
        initialise_camera(world);
    }
}

// Initialises

fn initialise_creatures(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut rng = rand::thread_rng();

    {
        // Plants

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 2,
        };

        for _ in 0..1000 {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                rng.gen_range(0.0, GROUND_WIDTH),
                rng.gen_range(0.0, GROUND_HEIGHT),
                0.0,
            );

            world
                .create_entity()
                .with(sprite_render.clone())
                .with(Velocity { x: 0.0, y: 0.0 })
                .with(Acceleration {
                    max_speed: PLANT_MAX_SPEED,
                    x: 0.0,
                    y: 0.0,
                })
                .with(Plant {
                    life: rng.gen_range(0, (PLANT_MIN_LIFE + PLANT_MAX_LIFE) / 2),
                    nutrition: rng.gen_range(0, PLANT_INITIAL_NUTRITION),
                })
                .with(transform)
                .build();
        }
    }

    {
        // Herbivore

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 1,
        };

        for _ in 0..500 {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                rng.gen_range(0.0, GROUND_WIDTH),
                rng.gen_range(0.0, GROUND_HEIGHT),
                0.0,
            );

            world
                .create_entity()
                .with(sprite_render.clone())
                .with(Velocity { x: 0.0, y: 0.0 })
                .with(Acceleration {
                    max_speed: HERBIVORE_MAX_SPEED,
                    x: 0.0,
                    y: 0.0,
                })
                .with(Herbivore {
                    life: rng.gen_range(0, (HERBIVORE_MIN_LIFE + HERBIVORE_MAX_LIFE) / 2),
                    nutrition: rng.gen_range(0, HERBIVORE_INITIAL_NUTRITION),
                })
                .with(transform)
                .build();
        }
    }

    {
        // Carnivore

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        };

        for _ in 0..50 {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                rng.gen_range(0.0, GROUND_WIDTH),
                rng.gen_range(0.0, GROUND_HEIGHT),
                0.0,
            );

            world
                .create_entity()
                .with(sprite_render.clone())
                .with(Velocity { x: 0.0, y: 0.0 })
                .with(Acceleration {
                    max_speed: CARNIVORE_MAX_SPEED,
                    x: 0.0,
                    y: 0.0,
                })
                .with(Carnivore {
                    life: rng.gen_range(0, (CARNIVORE_MIN_LIFE + CARNIVORE_MAX_LIFE) / 2),
                    nutrition: rng.gen_range(0, CARNIVORE_INITIAL_NUTRITION),
                })
                .with(transform)
                .build();
        }
    }
}

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
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/creature.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
