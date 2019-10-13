use crate::types::*;
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter, RonFormat},
    core::transform::Transform,
    ecs::{Entity, Read, ReadExpect, Write, WriteStorage},
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

#[derive(Default)]
pub struct MyState {
    progress_counter: ProgressCounter,
    target_entity: Vec<Entity>,
    setuped: bool,
}

impl MyState {
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
                let mut anim_key = PlayAnimationKey::<String>::new();
                anim_key.set_key(("sample".into(), 0, 10));
                let anim_time = AnimationTime::new();
                let mut transform = Transform::default();
                transform.set_scale([-0.5, 0.5, 1.0].into());
                transform.set_translation_x(-200.);
                transform.set_translation_y(-200.);

                self.target_entity.push(
                    data.world
                        .create_entity()
                        .with(transform)
                        .with(anim_key)
                        .with(anim_time)
                        .build(),
                );

                let mut anim_key = PlayAnimationKey::<String>::new();
                anim_key.set_key(("sample".into(), 0, 10));
                let anim_time = AnimationTime::new();
                let mut transform = Transform::default();
                transform.set_scale([0.5, 0.5, 1.0].into());
                transform.set_translation_x(200.);
                transform.set_translation_y(-200.);

                self.target_entity.push(
                    data.world
                        .create_entity()
                        .with(transform)
                        .with(anim_key)
                        .with(anim_time)
                        .build(),
                );

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
                Some((VirtualKeyCode::Up, ElementState::Pressed)) => {
                    world.exec(|mut time: WriteStorage<AnimationTime>| {
                        for e in &self.target_entity {
                            if let Some(time) = time.get_mut(*e) {
                                time.set_speed(0.);
                                time.add_second(1. / 60.);
                                println!("time: {}", time.current_time() * 60.);
                            }
                        }
                    });
                }
                Some((VirtualKeyCode::Down, ElementState::Pressed)) => {
                    world.exec(|mut time: WriteStorage<AnimationTime>| {
                        for e in &self.target_entity {
                            if let Some(time) = time.get_mut(*e) {
                                time.set_speed(0.);
                                time.add_second(-1. / 60.);
                                println!("time: {}", time.current_time() * 60.);
                            }
                        }
                    });
                }
                Some((VirtualKeyCode::Space, ElementState::Pressed)) => {
                    world.exec(
                        |(mut key, mut time): (
                            WriteStorage<PlayAnimationKey<String>>,
                            WriteStorage<AnimationTime>,
                        )| {
                            for e in &self.target_entity {
                                if let Some(key) = key.get_mut(*e) {
                                    let new_key = match key.key() {
                                        Some((name, pack_id, id)) => {
                                            (name.clone(), pack_id, (id + 1) % 13)
                                        }
                                        None => ("sample".into(), 0, 0usize),
                                    };
                                    key.set_key(new_key);
                                }
                            }
                            for e in &self.target_entity {
                                if let Some(time) = time.get_mut(*e) {
                                    time.set_time(0.0);
                                }
                            }
                        },
                    );
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