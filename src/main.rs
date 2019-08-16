use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, PrefabLoaderSystemDesc, ProgressCounter, RonFormat},
    core::transform::{Transform, TransformBundle},
    input::StringBindings,
    prelude::*,
    renderer::{
        camera::Camera,
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    window::ScreenDimensions,
};
use amethyst_sprite_studio::SpriteAnimation;
use debug_system::{EntityCountSystem, PositionDrawSystem};

#[derive(Default)]
struct MyState {
    progress_counter: ProgressCounter,
    prefab: Option<Handle<Prefab<SpriteAnimation>>>,
    setuped: bool,
}

fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();
}

impl SimpleState for MyState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.prefab = data
            .world
            .exec(|loader: PrefabLoader<'_, SpriteAnimation>| {
                loader.load("test.ron", RonFormat, &mut self.progress_counter)
            })
            .into();

        self.setuped = false;

        initialise_camera(&mut data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            if self.setuped == false {
                log::info!("complete!");
                data.world
                    .create_entity()
                    .with(self.prefab.as_ref().unwrap().clone())
                    .build();
                self.setuped = true;
            }
        }
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources_dir = app_root.join("resources");
    let display_config_path = resources_dir.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_system_desc(PrefabLoaderSystemDesc::<SpriteAnimation>::default(), "", &[])
        .with(EntityCountSystem::new(), "", &[])
        .with(PositionDrawSystem::new(), "", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                ),
        )?;

    let mut game = Application::new(resources_dir, MyState::default(), game_data)?;
    game.run();

    Ok(())
}
