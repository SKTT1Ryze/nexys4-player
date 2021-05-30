//! 华中科技大学接口技术大作业
mod player;
mod buttle;
mod manager;
mod config;

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use rand::Rng;
use config::*;

fn main() {
    App::build()
        .add_resource(manager::AssetsManager::empty())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(keyboard_event_system.system())
        .add_system(gravity_system.system())
        .add_system(buttle_move_system.system())
        .add_system(collision_system.system())
        .run();
}

#[derive(Default)]
struct State {
    keyborad_reader: EventReader<KeyboardInput>
}

struct Player;

struct Velocity {
    translation: Vec3,
    rotation: f32
}

impl std::default::Default for Velocity {
    fn default() -> Self {
        Self {
            translation: Vec3::new(100.0, 0.0, 0.0),
            rotation: -1.0
        }
    }
}

impl Velocity {
    pub fn new(translation: Vec3, rotation: f32) -> Self {
        Self {
            translation,
            rotation
        }
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let next_index = if (sprite.index + 1) % 3 == 0 {sprite.index - 2} else {sprite.index + 1};
            sprite.index = next_index;
        }
    }
}

// 通过键盘控制 Player0
fn keyboard_event_system(
    commands: &mut Commands,
    mut state: Local<State>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    assets_mananger: Res<manager::AssetsManager>,
    mut query: Query<(&mut Player, &mut Transform, &mut TextureAtlasSprite)>,
) {
    for event in state.keyborad_reader.iter(&keyboard_input_events) {
        // println!("{:?}", event);
        if !event.state.is_pressed() { return; }
        match event.key_code {
            Some(k) => {
                let (_, mut transform, mut sprite) = query.iter_mut().next().expect("query empty");
                match k {
                    KeyCode::W => {
                        // 改变 sprite 的位置
                        transform.translation.y += MAP_BLOCK_WIDTH;
                        // 改变 sprite 的朝向
                        sprite.index = 9;
                    },
                    KeyCode::S => {
                        transform.translation.y -= MAP_BLOCK_WIDTH;
                        sprite.index = 0;
                    },
                    KeyCode::A => {
                        transform.translation.x -= MAP_BLOCK_WIDTH;
                        sprite.index = 3;
                    },
                    KeyCode::D => {
                        transform.translation.x += MAP_BLOCK_WIDTH;
                        sprite.index = 6;
                    },
                    KeyCode::J => {
                        // 生成子弹
                        // todo: 通过名字来找相应的贴图
                        let texture = assets_mananger.texture.get(2).expect("failed to find texture");
                        let buttle = buttle::ButtleBuilder::type0(texture.clone());
                        todo!()
                    },
                    _ => {
                        // do nothing
                    }
                }
            },
            _ => {} // do nothing
        }
    }
}

/// 重力系统
fn gravity_system(time: Res<Time>, mut q: Query<Mut<Velocity>>) {
    let delta = time.delta_seconds();
    for mut v in q.iter_mut() {
        v.translation += Vec3::new(0.0, GRAVITY * delta * 2., 0.0);
    }
}

fn buttle_move_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, Mut<Transform>)>
) {
    let delta = time.delta_seconds();
    for (v, mut t) in query.iter_mut() {
        t.translation += delta * v.translation * 2.;
        t.rotate(Quat::from_rotation_z(v.rotation * delta));
    }
}

/// 碰撞系统
fn collision_system(
    wins: Res<Windows>,
    mut query: Query<(Mut<Velocity>, Mut<Transform>)>
) {
    let mut rnd = rand::thread_rng();
    let win = wins.get_primary().unwrap();

    let ceiling = win.height() / 2.;
    let ground = -(win.height() / 2.);
    let wall_left = -(win.width() / 2.);
    let wall_right = win.width() / 2.;

    for (mut v, mut t) in query.iter_mut() {
        let left = t.translation.x - BULLET_SIZE / 2.;
        let right = t.translation.x + BULLET_SIZE / 2.;
        let top = t.translation.y + BULLET_SIZE / 2.;
        let bottom = t.translation.y - BULLET_SIZE / 2.;

        // clamp the translation to not go out of the bounds
        if bottom < ground {
            t.translation.y = ground + BULLET_SIZE / 2.0;
            // apply an impulse upwards
            v.translation.y = rnd.gen_range(70.0, 100.0);
        }
        if top > ceiling {
            t.translation.y = ceiling - BULLET_SIZE / 2.0;
        }
        // on side walls flip the horizontal velocity
        if left < wall_left {
            t.translation.x = wall_left + BULLET_SIZE / 2.0;
            v.translation.x *= -1.0;
            v.rotation *= -1.0;
        }
        if right > wall_right {
            t.translation.x = wall_right - BULLET_SIZE / 2.0;
            v.translation.x *= -1.0;
            v.rotation *= -1.0;
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut assets_manager: ResMut<manager::AssetsManager>
) {
    // 生成 player
    let texture_handle0 = asset_server.load("textures/player0.png");
    let texture_handle1 = asset_server.load("textures/player1.png");
    assets_manager.texture.push(texture_handle0.clone());
    assets_manager.texture.push(texture_handle1.clone());
    let player0 = player::PlayerBuilder::default0(
        texture_handle0.clone(), player::TextureSize::new(3, 4)
    );
    let player1 = player::PlayerBuilder::default1(
        texture_handle1.clone(), player::TextureSize::new(3, 4)
    );
    let texture_atlas0 = TextureAtlas::from_grid(player0.texture.0.clone(), player0.size, player0.texture.1.columns, player0.texture.1.rows);
    let texture_atlas1 = TextureAtlas::from_grid(player1.texture.0.clone(), player1.size, player1.texture.1.columns, player1.texture.1.rows);
    let texture_atlas_handle0 = texture_atlases.add(texture_atlas0);
    let texture_atlas_handle1 = texture_atlases.add(texture_atlas1);
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle0,
            transform: player0.transform,
            ..Default::default()
        })
        .with(Timer::from_seconds(TIMER_INTERVAL, true))
        .with(Player)
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle1,
            transform: player1.transform,
            ..Default::default()
        })
        .with(Timer::from_seconds(TIMER_INTERVAL, true))
        .with(Player);

    // 生成子弹
    let texture_handle = asset_server.load("branding/icon.png");
    assets_manager.texture.push(texture_handle.clone());
    let buttle = buttle::ButtleBuilder::type0(texture_handle);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                size: buttle.size,
                resize_mode: SpriteResizeMode::Manual
            },
            material: materials.add(
                ColorMaterial {
                    color: buttle.color,
                    texture: Some(buttle.texture.clone())
                }
            ),
            transform: Transform {
                scale: Vec3 {x: SPRITE_SCALE / 2.0, y: SPRITE_SCALE / 2.0, z: 0.0},
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Velocity::default());
}