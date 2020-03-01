mod components;
mod resources;
mod state;
mod types;

use crate::{state::example::MyState, types::*};
use amethyst::{
    assets::{HotReloadBundle, HotReloadStrategy},
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
    LoggerConfig,
};
use amethyst_collision::bundle::CollisionSystemBundle;
use amethyst_sprite_studio::{bundle::SpriteStudioBundle, renderer::RenderSpriteAnimation};
use debug_system::DebugSystemBundle;
use fight_game::{
    bundle::FightGameBundle,
    components::Collisions,
    id::{
        file::FileId,
        pack::{AnimationKey, PackKey},
    },
    input::FightInput,
    paramater::{Aabb, CollisionParamater},
};
use input_handle::traits::InputParser;

fn main() -> amethyst::Result<()> {
    let logger_config = LoggerConfig {
        level_filter: amethyst::LogLevelFilter::Info,
        log_file: Some("log/log.txt".into()),
        ..Default::default()
    };
    amethyst::Logger::from_config(logger_config)
        .level_for("debug_collision", amethyst::LogLevelFilter::Info)
        .level_for("resource::animation", amethyst::LogLevelFilter::Trace)
        .level_for(
            "amethyst_collision::system::detect_contact",
            amethyst::LogLevelFilter::Info,
        )
        .level_for("fight_game", amethyst::LogLevelFilter::Error)
        .level_for("fight_game::load", amethyst::LogLevelFilter::Info)
        .level_for("fight_game::input", amethyst::LogLevelFilter::Info)
        .level_for(
            "fight_game::components::collision",
            amethyst::LogLevelFilter::Info,
        )
        .level_for("sync_position_to_world", amethyst::LogLevelFilter::Error)
        .level_for(
            "fight_game::components::collision",
            amethyst::LogLevelFilter::Error,
        )
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Warn)
        .start();
    let app_root = application_root_dir()?;
    log::info!("{:?}", app_root);
    let resources_dir = app_root.join("resources");
    let display_config_path = resources_dir.join("display_config.ron");
    let input_config_path = resources_dir.join("config").join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::every(10)))?
        .with_bundle(
            InputBundle::<<FightInput as InputParser>::BindingTypes>::new()
                .with_bindings_from_file(input_config_path)
                .unwrap(),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<<FightInput as InputParser>::BindingTypes>::new())?
        .with_bundle(SpriteStudioBundle::<
            types::translate_animation::FightTranslation,
        >::new())?
        .with_bundle(FightGameBundle::<FileId, PackKey, AnimationKey, Aabb, ()>::new())?
        .with_bundle(CollisionSystemBundle::<
            Collisions<Aabb, ()>,
            CollisionParamater,
        >::new(true))?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(DebugSystemBundle::new())?
        .with_barrier()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderSpriteAnimation::<
                    FileId,
                    PackKey,
                    AnimationKey,
                    UserData,
                >::default())
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
