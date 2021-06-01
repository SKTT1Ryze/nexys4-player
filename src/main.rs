//! 华中科技大学接口技术大作业
mod player;
mod buttle;
mod manager;
mod config;

use bevy::math::f32;
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
    mut query: Query<(&mut player::Player, &mut Transform, &mut TextureAtlasSprite)>,
) {
    for event in state.keyborad_reader.iter(&keyboard_input_events) {
        // println!("{:?}", event);
        if !event.state.is_pressed() { return; }
        match event.key_code {
            Some(k) => {
                // 取出第一个 player 对其进行控制
                let (mut player, mut transform, mut sprite) = query.iter_mut().next().expect("query empty");
                match k {
                    KeyCode::W => {
                        // 改变 sprite 的位置
                        transform.translation.y += MAP_BLOCK_WIDTH;
                        // 改变 sprite 的朝向
                        sprite.index = 9;
                        // 改变 player 的朝向记录
                        player.toward = player::TOWARD::Up;
                    },
                    KeyCode::S => {
                        transform.translation.y -= MAP_BLOCK_WIDTH;
                        sprite.index = 0;
                        player.toward = player::TOWARD::Down;
                    },
                    KeyCode::A => {
                        transform.translation.x -= MAP_BLOCK_WIDTH;
                        sprite.index = 3;
                        player.toward = player::TOWARD::Left;
                    },
                    KeyCode::D => {
                        transform.translation.x += MAP_BLOCK_WIDTH;
                        sprite.index = 6;
                        player.toward = player::TOWARD::Right;
                    },
                    KeyCode::J => {
                        // 生成子弹
                        // todo: 通过名字来找相应的贴图
                        // 如果角色不是水平朝向不能发射子弹
                        if player.toward == player::TOWARD::Up || player.toward == player::TOWARD::Down { return; }
                        let texture = assets_mananger.textures.get("buttle").expect("failed to find texture");
                        let buttle = buttle::ButtleBuilder::type0(texture.clone());
                        let material = assets_mananger.materials.get("red").expect("failed to find material");
                        let mut buttle_transform = transform.clone();
                        buttle_transform.scale.x /= 2.;
                        buttle_transform.scale.y /= 2.;
                        let mut velocity = Velocity::default();
                        if player.toward == player::TOWARD::Left {
                            velocity.translation *= -1.;
                        }
                        commands
                            .spawn(
                                SpriteBundle {
                                    sprite: Sprite {
                                        size: buttle.size,
                                        resize_mode: SpriteResizeMode::Manual
                                    },
                                    material: material.clone(),
                                    transform: buttle_transform,
                                    ..Default::default()
                                }
                            )
                            .with(velocity)
                            .with(buttle::Buttle::new(0, 5));
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
    commands: &mut Commands,
    wins: Res<Windows>,
    mut buttle_query: Query<(Entity, Mut<Velocity>, Mut<Transform>, Mut<buttle::Buttle>)>,
    mut player_query: Query<(Mut<player::Player>, Mut<Transform>)>
) {
    let mut rnd = rand::thread_rng();
    let win = wins.get_primary().unwrap();

    let ceiling = win.height() / 2.;
    let ground = -(win.height() / 2.);
    let wall_left = -(win.width() / 2.);
    let wall_right = win.width() / 2.;

    for (e, mut v, mut t, mut b) in buttle_query.iter_mut() {
        let left = t.translation.x - BULLET_SIZE / 2.;
        let right = t.translation.x + BULLET_SIZE / 2.;
        let top = t.translation.y + BULLET_SIZE / 2.;
        let bottom = t.translation.y - BULLET_SIZE / 2.;

        // clamp the translation to not go out of the bounds
        if bottom < ground {
            t.translation.y = ground + BULLET_SIZE / 2.0;
            // apply an impulse upwards
            v.translation.y = rnd.gen_range(70.0, 100.0);
            if b.crash() {
                commands.despawn(e);
            }
        }
        if top > ceiling {
            t.translation.y = ceiling - BULLET_SIZE / 2.0;
            commands.despawn(e);
        }
        // on side walls flip the horizontal velocity
        if left < wall_left {
            t.translation.x = wall_left + BULLET_SIZE / 2.0;
            v.translation.x *= -1.0;
            v.rotation *= -1.0;
            if b.crash() {
                commands.despawn(e);
            }
        }
        if right > wall_right {
            t.translation.x = wall_right - BULLET_SIZE / 2.0;
            v.translation.x *= -1.0;
            v.rotation *= -1.0;
            if b.crash() {
                commands.despawn(e);
            }
        }

        for (mut p, t) in player_query.iter_mut() {
            let player_left = t.translation.x - PLAYER_SIZE / 2.;
            let player_right = t.translation.x + PLAYER_SIZE / 2.;
            let player_top = t.translation.y + PLAYER_SIZE / 2.;
            let player_bottom = t.translation.y - PLAYER_SIZE / 2.;
            if is_intersected(
                left, right, top, bottom,
                player_left, player_right, player_top, player_bottom
            ) {
                println!("buttle crash player!");
                p.crash();
            }

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
    assets_manager.textures.insert("player0".to_string(), texture_handle0.clone());
    assets_manager.textures.insert("player1".to_string(), texture_handle1.clone());
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
        .with(player::Player::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle1,
            transform: player1.transform,
            ..Default::default()
        })
        .with(Timer::from_seconds(TIMER_INTERVAL, true))
        .with(player::Player::default());
    
    // 生成子弹
    let texture_handle = asset_server.load("branding/icon.png");
    assets_manager.textures.insert("buttle".to_string(), texture_handle.clone());
    let buttle0 = buttle::ButtleBuilder::type0(texture_handle.clone());
    let buttle1 = buttle::ButtleBuilder::type1(texture_handle);
    let material = materials.add(
        ColorMaterial {
            color: buttle0.color,
            texture: Some(buttle0.texture.clone())
        }
    );
    assets_manager.materials.insert("red".to_string(), material.clone());
    let material = materials.add(
        ColorMaterial {
            color: buttle1.color,
            texture: Some(buttle1.texture.clone())
        }
    );
    assets_manager.materials.insert("blue".to_string(), material.clone());
}

/// 判断两个矩形是否相交
fn is_intersected(
    left_x: f32, right_x: f32, top_x: f32, bottom_x: f32,
    left_y: f32, right_y: f32, top_y: f32, bottom_y: f32
) -> bool {
    let left = left_x.max(left_y);
    let bottom = bottom_x.max(bottom_y);
    let right = right_x.min(right_y);
    let top = top_x.min(top_y);
    !(left > right || bottom > top)
}