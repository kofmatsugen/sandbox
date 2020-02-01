use crate::types::*;
use amethyst::{
    assets::ProgressCounter,
    core::transform::Transform,
    ecs::{BitSet, Entity, Join, WorldExt as _, WriteStorage},
    input::{get_key, VirtualKeyCode},
    prelude::*,
    renderer::{camera::Camera, ActiveCamera},
    shred::World,
    window::ScreenDimensions,
    winit::ElementState,
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    resource::WorldExt,
};
use fight_game::id::{
    file::FileId,
    pack::{AnimationKey, PackKey},
};

const DEFAULT_SPEED: f32 = 1.;

#[derive(Default)]
pub struct MyState {
    progress_counter: ProgressCounter,
    target_entity: BitSet,
    setuped: bool,
}

impl MyState {
    fn on_pressed_key(&mut self, world: &mut World, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Up => {
                world.exec(|mut time: WriteStorage<AnimationTime>| {
                    for (_, time) in (&self.target_entity, &mut time).join() {
                        time.set_speed(0.);
                        time.add_second(1. / 60.);
                        log::info!("time: {}", time.current_time() * 60.);
                    }
                });
            }
            VirtualKeyCode::Down => {
                world.exec(|mut time: WriteStorage<AnimationTime>| {
                    for (_, time) in (&self.target_entity, &mut time).join() {
                        time.set_speed(0.);
                        time.add_second(-1. / 60.);
                        log::info!("time: {}", time.current_time() * 60.);
                    }
                });
            }
            VirtualKeyCode::Space => {
                world.exec(
                    |(_key, _time): (
                        WriteStorage<PlayAnimationKey<FileId, PackKey, AnimationKey>>,
                        WriteStorage<AnimationTime>,
                    )| {},
                );
            }
            VirtualKeyCode::Left => {
                world.exec(|mut transforms: WriteStorage<Transform>| {
                    for (_, transform) in (&self.target_entity, &mut transforms).join().take(1) {
                        transform.append_translation_xyz(-1., 0., 0.);
                        log::info!("to: {:?}", transform.translation());
                    }
                });
            }
            VirtualKeyCode::Right => {
                world.exec(|mut transforms: WriteStorage<Transform>| {
                    for (_, transform) in (&self.target_entity, &mut transforms).join().take(1) {
                        transform.append_translation_xyz(1., 0., 0.);
                        log::info!("to: {:?}", transform.translation());
                    }
                });
            }
            VirtualKeyCode::Escape => {
                world.exec(|entities: amethyst::ecs::Entities| {
                    for (_, e) in (&self.target_entity, &*entities).join() {
                        let _ = entities.delete(e);
                    }
                });
            }
            _ => {}
        }
    }

    fn load_animation<W: WorldExt>(&mut self, world: &mut W) {
        world.load_animation_files::<FileId, UserData, PackKey, AnimationKey>(
            FileId::Sample,
            &mut self.progress_counter,
        );
    }
}

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("start simple state");
        let StateData { mut world, .. } = data;
        self.setuped = false;

        self.load_animation(&mut world);

        initialise_camera(&mut world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            if self.setuped == false {
                self.target_entity
                    .add(create_unit(data.world, (-200., -200.), (-0.5, 0.5)).id());

                log::info!("complete!");
                self.setuped = true;
            }
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = _data;
        if let StateEvent::Window(event) = &event {
            match get_key(&event) {
                Some((key, ElementState::Pressed)) => {
                    self.on_pressed_key(world, key);
                }
                _ => {}
            }
        }
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

fn create_unit<V2>(world: &mut World, position: V2, scale: V2) -> Entity
where
    V2: Into<Option<(f32, f32)>>,
{
    let (pos_x, pos_y) = position.into().unwrap_or((0., 0.));
    let (scale_x, scale_y) = scale.into().unwrap_or((1., 1.));
    let mut anim_key = PlayAnimationKey::<FileId, PackKey, AnimationKey>::new(FileId::Sample);
    anim_key.set_pack(PackKey::Base);
    anim_key.set_animation(AnimationKey::Stance);
    let mut anim_time = AnimationTime::new();
    anim_time.set_speed(DEFAULT_SPEED);
    let mut transform = Transform::default();
    transform.set_scale([scale_x, scale_y, 1.0].into());
    transform.set_translation_x(pos_x);
    transform.set_translation_y(pos_y);

    world
        .create_entity()
        .with(transform)
        .with(anim_key)
        .with(anim_time)
        .build()
}
