mod prefab;
mod state;

use crate::state::example::MyState;
use amethyst::{
    assets::PrefabLoaderSystemDesc,
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
use amethyst_aabb::bundle::AabbCollisionBundle;
use amethyst_sprite_studio::{
    bundle::SpriteStudioBundle,
    renderer::RenderSpriteAnimation,
    splash::{SplashState, SplashTranslation},
};
use debug_system::DebugSystemBundle;
#[cfg(feature = "debug")]
use fight_game::types::debug::DisplayInfo;
use fight_game::{
    bundle::{FightCollisionBundle, FightParamaterBundle, FightTransformBundle},
    components::HitInfo,
    input::FightInput,
    paramater::{CollisionParamater, FightTranslation},
};
use input_handle::traits::InputParser;
use movement_transform::system::TransformMovementSystem;
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
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(
            InputBundle::<<FightInput as InputParser>::BindingTypes>::new()
                .with_bindings_from_file(input_config_path)
                .unwrap(),
        )?
        // 前のフレームで発生した格闘ゲーム関連の判定情報，移動情報を反映
        .with_bundle(FightTransformBundle::<
            FightTranslation,
            CollisionParamater<FightTranslation>,
            HitInfo<FightTranslation>,
        >::new())?
        .with_barrier()
        // 前のフレームで発生した全移動情報を反映
        .with(TransformMovementSystem::new(), "movement_transform", &[])
        .with_bundle(TransformBundle::new())?
        .with_barrier()
        // 移動処理を終えてからアニメーション系の処理を行う
        .with_bundle(SpriteStudioBundle::<FightTranslation>::new())?
        .with_barrier()
        // 移動とアニメーションノードの作成情報を反映
        .with_bundle(FightParamaterBundle::<
            FightTranslation,
            CollisionParamater<FightTranslation>,
            HitInfo<FightTranslation>,
        >::new())?
        .with_barrier()
        .with_bundle(UiBundle::<<FightInput as InputParser>::BindingTypes>::new())?
        .with_bundle(AabbCollisionBundle::<CollisionParamater<FightTranslation>>::new())?
        .with_barrier()
        // 移動とアニメーションノードの作成情報を反映
        .with_bundle(FightCollisionBundle::<
            FightTranslation,
            CollisionParamater<FightTranslation>,
            HitInfo<FightTranslation>,
        >::new())?
        .with_barrier()
        // すべてのバンドル処理を終えたあとにデバッグ情報は追加する
        .with_bundle(DebugSystemBundle::<DisplayInfo>::new())?
        .with_barrier()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderSpriteAnimation::<FightTranslation>::default())
                .with_plugin(RenderSpriteAnimation::<SplashTranslation>::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                ),
        )?;

    let mut game = Application::new(resources_dir, SplashState::<MyState>::new(), game_data)?;
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
