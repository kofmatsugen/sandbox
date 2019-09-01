use amethyst::{
    assets::{AssetStorage, Loader, Processor, ProgressCounter, RonFormat},
    core::transform::{Transform, TransformBundle},
    ecs::{Read, ReadExpect, Write},
    input::StringBindings,
    prelude::*,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        sprite::{SpriteSheet, SpriteSheetFormat},
        types::DefaultBackend,
        RenderingBundle, Texture,
    },
    shred::World,
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    window::ScreenDimensions,
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    resource::AnimationStore,
    system::{AnimationTimeIncrementSystem, TimeLineApplySystem},
    timeline::SpriteAnimation,
};
use debug_system::{EntityCountSystem, PositionDrawSystem};

type Animation = SpriteAnimation<()>;

#[derive(Default)]
struct MyState {
    progress_counter: ProgressCounter,
    setuped: bool,
}

fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(100.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();
}

impl MyState {
    fn load_animation(
        &mut self,
        world: &mut World,
        pack_name: &str,
        sprite_num: usize,
        anim_num: usize,
    ) {
        let animation = world.exec(
            |(loader, storage): (ReadExpect<Loader>, Read<AssetStorage<Animation>>)| {
                let mut animation = vec![];
                for i in 0..anim_num {
                    let handle = loader.load(
                        format!("{}/animation/animation{:03}.anim.ron", pack_name, i),
                        RonFormat,
                        &mut self.progress_counter,
                        &storage,
                    );
                    animation.push(handle);
                }
                animation
            },
        );

        let sprite_sheet = world.exec(
            |(loader, tex_storage, sprite_storage): (
                ReadExpect<Loader>,
                Read<AssetStorage<Texture>>,
                Read<AssetStorage<SpriteSheet>>,
            )| {
                let mut sprite_sheets = vec![];
                for i in 0..sprite_num {
                    let texture = loader.load(
                        format!("{}/image/sprite{:03}.png", pack_name, i),
                        ImageFormat::default(),
                        &mut self.progress_counter,
                        &tex_storage,
                    );
                    let sheet = loader.load(
                        format!("{}/sheet/sprite{:03}.sheet.ron", pack_name, i),
                        SpriteSheetFormat(texture),
                        &mut self.progress_counter,
                        &sprite_storage,
                    );
                    sprite_sheets.push(sheet);
                }
                sprite_sheets
            },
        );

        world.exec(|mut anim_data: Write<AnimationStore<String, ()>>| {
            for anim in animation {
                anim_data.insert_animation(pack_name, anim);
            }
            for sheet in sprite_sheet {
                anim_data.insert_sprite_sheet(pack_name, sheet);
            }
        });
    }
}

impl SimpleState for MyState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.setuped = false;

        self.load_animation(&mut data.world, "houou", 1, 3);

        initialise_camera(&mut data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            if self.setuped == false {
                let mut anim_key = PlayAnimationKey::<String>::new();
                anim_key.set_key(("houou".into(), 0usize));

                let anim_time = AnimationTime::new();

                let mut transform = Transform::default();
                transform.set_scale([2.0, 2.0, 1.0].into());

                data.world
                    .create_entity()
                    .with(transform)
                    .with(anim_key)
                    .with(anim_time)
                    .build();

                log::info!("complete!");
                self.setuped = true;
            }
        }
        Trans::None
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources_dir = app_root.join("resources");
    let display_config_path = resources_dir.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(EntityCountSystem::new(), "", &[])
        .with(PositionDrawSystem::new(), "", &[])
        .with(Processor::<Animation>::new(), "", &[])
        .with(TimeLineApplySystem::<String, ()>::new(), "", &[])
        .with(AnimationTimeIncrementSystem::new(), "", &[])
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
