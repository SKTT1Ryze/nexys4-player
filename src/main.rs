//! 华中科技大学接口技术大作业
mod player;
mod config;

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use config::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(keyboard_event_system.system())
        .run();
}

#[derive(Default)]
struct State {
    keyborad_reader: EventReader<KeyboardInput>
}

struct Player;

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let next_index = if (sprite.index + 1) % 3 == 0 {sprite.index - 2} else {sprite.index + 1};
            sprite.index = next_index;
        }
    }
}

// 通过键盘控制 Player0
fn keyboard_event_system(
    mut state: Local<State>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
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
                        transform.translation.y += MOVE_STEP;
                        // 改变 sprite 的朝向
                        sprite.index = 9;
                    },
                    KeyCode::S => {
                        transform.translation.y -= MOVE_STEP;
                        sprite.index = 0;
                    },
                    KeyCode::A => {
                        transform.translation.x -= MOVE_STEP;
                        sprite.index = 3;
                    },
                    KeyCode::D => {
                        transform.translation.x += MOVE_STEP;
                        sprite.index = 6;
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

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle0 = asset_server.load("textures/player0.png");
    let texture_handle1 = asset_server.load("textures/player1.png");
    let player0 = player::PlayerBuilder::default0(
        texture_handle0.clone(), player::TextureSize::new(3, 4)
    );
    let player1 = player::PlayerBuilder::default1(
        texture_handle1.clone(), player::TextureSize::new(3, 4)
    );
    let texture_atlas0 = TextureAtlas::from_grid(player0.texture.0.clone(), player0.tile_size, player0.texture.1.columns, player0.texture.1.rows);
    let texture_atlas1 = TextureAtlas::from_grid(player1.texture.0.clone(), player1.tile_size, player1.texture.1.columns, player1.texture.1.rows);
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
}