use crate::types::*;
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter, RonFormat},
    core::transform::Transform,
    ecs::{BitSet, Entity, Join, Read, ReadExpect, Write, WriteStorage},
    input::{get_key, VirtualKeyCode},
    prelude::*,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat},
        ActiveCamera, Texture,
    },
    shred::World,
    window::ScreenDimensions,
    winit::ElementState,
};
use amethyst_sprite_studio::{
    components::{AnimationTime, PlayAnimationKey},
    resource::AnimationStore,
};

const DEFAULT_SPEED: f32 = 0.5;

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
                    |(mut key, mut time): (
                        WriteStorage<PlayAnimationKey<String>>,
                        WriteStorage<AnimationTime>,
                    )| {
                        for (_, key, time) in (&self.target_entity, &mut key, &mut time).join() {
                            let new_key = match key.key() {
                                Some((name, pack_id, id)) => (name.clone(), pack_id, (id + 1) % 3),
                                None => ("sample".into(), 0, 0usize),
                            };
                            key.set_key(new_key);
                            time.set_time(0.0);
                            time.set_speed(DEFAULT_SPEED);
                        }
                    },
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
            _ => {}
        }
    }

    fn load_animation(
        &mut self,
        world: &mut World,
        pack_name: &str,
        sprite_num: usize,
        anim_nums: Vec<usize>,
    ) {
        let animation = world.exec(
            |(loader, storage): (ReadExpect<Loader>, Read<AssetStorage<Animation>>)| {
                let mut pack = vec![];
                for (pack_idx, anim_num) in anim_nums.into_iter().enumerate() {
                    let mut animation = vec![];
                    for i in 0..anim_num {
                        let handle = loader.load(
                            format!(
                                "{}/animation/pack{:03}/animation{:03}.anim.ron",
                                pack_name, pack_idx, i
                            ),
                            RonFormat,
                            &mut self.progress_counter,
                            &storage,
                        );
                        animation.push(handle);
                    }
                    pack.push(animation);
                }
                pack
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

        world.exec(|mut anim_data: Write<AnimationStore<String, UserData>>| {
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

        self.load_animation(&mut data.world, "sample", 1, vec![13]);

        initialise_camera(&mut data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            if self.setuped == false {
                self.target_entity
                    .add(create_unit(data.world, "sample", 0, 7, (-200., -200.), (-0.5, 0.5)).id());
                self.target_entity
                    .add(create_unit(data.world, "sample", 0, 0, (200., -200.), (0.45, 0.45)).id());

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

fn create_unit<S, V2>(
    world: &mut World,
    file_name: S,
    pack_id: usize,
    anim_id: usize,
    position: V2,
    scale: V2,
) -> Entity
where
    S: Into<String>,
    V2: Into<Option<(f32, f32)>>,
{
    let (pos_x, pos_y) = position.into().unwrap_or((0., 0.));
    let (scale_x, scale_y) = scale.into().unwrap_or((1., 1.));
    let mut anim_key = PlayAnimationKey::<String>::new();
    anim_key.set_key((file_name.into(), pack_id, anim_id));
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
