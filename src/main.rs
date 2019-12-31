extern crate amethyst;
mod earth;
mod systems;

use amethyst::{
    core::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    utils::application_root_dir
};
use crate::earth::Earth;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let asset_dir = app_root.join("assets/");

    // Creature
    // ↓
    // Solar
    // ↓
    // Plant-Herbivore
    // ↓
    // Herbivore-Carnivore
    // ↓
    // Plant, Carnivore, Herbivore
    
    // Plant, Herbivore, Carnivore, PlantIncubator, HerbivoreIncubator, CarnivoreIncubator -> mut acceleration
    //
    // PositionUpdater
    //
    // Solar
    // Plant_Herbivore
    // Herbivore_Carnivore

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with(systems::PlantIncubator, "plant_incubator", &[])
        .with(systems::PlantMechanics, "plant_mechanics", &["plant_incubator"])
        .with(systems::HerbivoreMechanics, "herbivore_mechanics", &[])
        .with(systems::PositionUpdater, "position_updater", &["plant_mechanics", "herbivore_mechanics"])
        .with(systems::SolarSystem, "solar_system", &["position_updater"])
        .with(systems::PlantHerbivore, "plant_herbivore", &["solar_system"])
        //.with(systems::PlantSystem, "plant_system", &["plant_herbivore_system"])
        //.with(systems::HerbivoreSystem, "herbivore_system", &["plant_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.5, 0.5, 0.5, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
        )?;

    let mut game = Application::new(asset_dir, Earth::default(), game_data)?;
    game.run();

    Ok(())
}
