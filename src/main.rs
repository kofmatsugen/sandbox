mod prefab;
mod state;

use crate::state::example::MyState;
use amethyst::{
    assets::{HotReloadBundle, HotReloadStrategy, PrefabLoaderSystemDesc},
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};
use amethyst_collision::bundle::CollisionSystemBundle;
use amethyst_sprite_studio::{bundle::SpriteStudioBundle, renderer::RenderSpriteAnimation};
use debug_system::DebugSystemBundle;
use fight_game::{
    bundle::FightGameBundle,
    components::Collisions,
    input::FightInput,
    paramater::{Aabb, CollisionParamater, ContactParamter, FightTranslation},
};
use input_handle::traits::InputParser;
use prefab::character::CharacterPrefab;

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;
    #[cfg(feature = "debug")]
    logger(&app_root).unwrap();

    let resources_dir = app_root.join("resources");
    let display_config_path = resources_dir.join("display_config.ron");
    let input_config_path = resources_dir.join("config").join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<CharacterPrefab>::default(),
            "character_prefab_loader",
            &[],
        )
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::every(10)))?
        .with_bundle(
            InputBundle::<<FightInput as InputParser>::BindingTypes>::new()
                .with_bindings_from_file(input_config_path)
                .unwrap(),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<<FightInput as InputParser>::BindingTypes>::new())?
        .with_bundle(SpriteStudioBundle::<FightTranslation>::new())?
        .with_bundle(CollisionSystemBundle::<
            Collisions<Aabb, CollisionParamater>,
            ContactParamter,
        >::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(FightGameBundle::<FightTranslation, Aabb, CollisionParamater>::new())?
        .with_bundle(DebugSystemBundle::new())?
        .with_barrier()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderSpriteAnimation::<FightTranslation>::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                ),
        )?;

    let mut game = Application::new(resources_dir, MyState::default(), game_data)?;
    game.run();

    Ok(())
}

#[cfg(feature = "debug")]
fn logger(root: &std::path::PathBuf) -> anyhow::Result<()> {
    use std::io::Read;
    let toml_file = root.join("debug").join("logger.toml");
    let mut string = String::new();
    let mut f = std::fs::File::open(toml_file)?;
    f.read_to_string(&mut string)?;
    let logger_config = toml::from_str(&string)?;

    amethyst::Logger::from_config(logger_config).start();

    Ok(())
}
