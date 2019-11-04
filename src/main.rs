mod components;
mod playfab;
mod resources;
mod state;
mod types;

use crate::{
    playfab::config::PlayFab,
    state::{example::MyState, playfab_check::PlayFabCheck},
    types::*,
};
use amethyst::{
    core::transform::TransformBundle,
    input::StringBindings,
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    LoggerConfig,
};
use amethyst_playfab::bundle::PlayFabSystemBundle;
use amethyst_sprite_studio::{bundle::SpriteStudioBundleBuilder, renderer::RenderSpriteAnimation};
use debug_system::{EntityCountSystem, PositionDrawSystem};
use fight_game::system::MoveSystem;

fn main() -> amethyst::Result<()> {
    let logger_config = LoggerConfig::default();
    amethyst::Logger::from_config(logger_config)
        .level_for("debug_collision", amethyst::LogLevelFilter::Debug)
        .start();
    let app_root = application_root_dir()?;

    let resources_dir = app_root.join("resources");
    let display_config_path = resources_dir.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(SpriteStudioBundleBuilder::with_debug_collision::<
            String,
            UserData,
        >())?
        .with(MoveSystem::<String>::new(), "move_system", &[])
        .with(EntityCountSystem::new(), "", &[])
        .with(PositionDrawSystem::new(), "", &[])
        .with_barrier()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderSpriteAnimation::<String, UserData>::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                ),
        )?
        .with_bundle(PlayFabSystemBundle::<PlayFab>::new())?;

    let mut game = Application::new(resources_dir, PlayFabCheck::<MyState>::default(), game_data)?;
    game.run();

    Ok(())
}
