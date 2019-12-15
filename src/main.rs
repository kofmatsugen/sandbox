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
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    LoggerConfig,
};
use amethyst_collision::bundle::CollisionSystemBundle;
use amethyst_playfab::bundle::PlayFabSystemBundle;
use amethyst_sprite_studio::{bundle::SpriteStudioBundleBuilder, renderer::RenderSpriteAnimation};
use debug_system::DebugSystemBundle;
use fight_game::{
    components::Collisions,
    paramater::{Aabb, CollisionParamater},
    system::{MoveSystem, RegisterColliderSystem},
};

fn main() -> amethyst::Result<()> {
    let logger_config = LoggerConfig::default();
    amethyst::Logger::from_config(logger_config)
        .level_for("debug_collision", amethyst::LogLevelFilter::Debug)
        .level_for(
            "amethyst_collision::system::detect_contact",
            amethyst::LogLevelFilter::Info,
        )
        .level_for("fight_game", amethyst::LogLevelFilter::Error)
        .level_for(
            "fight_game::components::collision",
            amethyst::LogLevelFilter::Info,
        )
        .level_for("sync_position_to_world", amethyst::LogLevelFilter::Error)
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
        .with_bundle(CollisionSystemBundle::<
            Collisions<Aabb, ()>,
            CollisionParamater,
        >::new())?
        .with(
            RegisterColliderSystem::<String, Aabb, ()>::new(),
            "register_collider",
            &[],
        )
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
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(PlayFabSystemBundle::<PlayFab>::new())?
        .with_bundle(DebugSystemBundle::new())?;

    let mut game = Application::new(resources_dir, PlayFabCheck::<MyState>::default(), game_data)?;
    game.run();

    Ok(())
}
