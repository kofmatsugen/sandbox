use crate::prefab::character::CharacterPrefab;
use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    core::transform::Transform,
    ecs::{BitSet, Entity, WorldExt},
    prelude::*,
    renderer::{camera::Camera, ActiveCamera},
    shred::World,
    ui::UiCreator,
    window::ScreenDimensions,
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    load::AnimationLoad,
};
use fight_game::{
    components::ActiveCommand,
    id::{
        file::FileId,
        pack::{AnimationKey, PackKey},
    },
    load::CommandLoad,
    paramater::FightTranslation,
};
use movement_transform::components::Movement;

#[cfg(feature = "debug")]
use debug_system::components::DebugInfomationDisplay;
#[cfg(feature = "debug")]
use fight_game::types::debug::DisplayInfo;

const DEFAULT_SPEED: f32 = 1.;

#[derive(Default)]
pub struct MyState {
    progress_counter: ProgressCounter,
    target_entity: BitSet,
    setuped: bool,
    character_prefab: Vec<Handle<Prefab<CharacterPrefab>>>,
}

impl MyState {
    fn load_animation<W: AnimationLoad>(&mut self, world: &mut W) {
        world.load_animation_files::<FightTranslation>(FileId::Sample, &mut self.progress_counter);
        world.load_animation_files::<FightTranslation>(FileId::Sandbox, &mut self.progress_counter);
    }

    fn load_command<W: CommandLoad>(&mut self, world: &mut W) {
        world.load_command("command", "basic", &mut self.progress_counter);
    }
}

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("start simple state");
        let StateData { mut world, .. } = data;
        self.setuped = false;

        self.load_animation(&mut world);
        self.load_command(&mut world);

        world.exec(|mut creator: UiCreator| {
            creator.create("debug/ui/debug_ui.ron", &mut self.progress_counter);
        });

        self.character_prefab = (0..2)
            .map(|i| {
                world.exec(|loader: PrefabLoader<CharacterPrefab>| {
                    loader.load(
                        format!("prefab/character/base_{}.ron", i),
                        RonFormat,
                        &mut self.progress_counter,
                    )
                })
            })
            .collect();

        initialise_camera(&mut world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            if self.setuped == false {
                for p in self.character_prefab.iter().cloned() {
                    self.target_entity.add(create_unit(data.world, p).id());
                }
                log::info!("complete!");
                self.setuped = true;
            }
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        _event: StateEvent,
    ) -> SimpleTrans {
        Trans::None
    }
    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}

fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1024.0);

    let camera = world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();

    world.insert(ActiveCamera {
        entity: Some(camera),
    });
}

fn create_unit(world: &mut World, character_prefab: Handle<Prefab<CharacterPrefab>>) -> Entity {
    let mut anim_key = PlayAnimationKey::<FightTranslation>::new(FileId::Sample);
    anim_key.set_pack(PackKey::Base);
    anim_key.set_animation(AnimationKey::Stance);
    let mut anim_time = AnimationTime::new();
    anim_time.play(DEFAULT_SPEED / 10.);

    world
        .create_entity()
        .with(anim_key)
        .with(anim_time)
        .with(ActiveCommand::new())
        .with(character_prefab)
        .with(Movement::new())
        .with(DebugInfomationDisplay::<DisplayInfo>::new())
        .build()
}
