// 华中科技大学接口技术大作业

mod serial;

use bevy::prelude::*;
use std::time::Duration;
use serial::Nexys4Serial;

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

struct SnakeSpawnTimer(Timer);
impl Default for SnakeSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(1000), true))
    }
}

// Commands -> Resources -> Components -> Queries
fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(
        Materials {
            head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into())
        }
    );
}

fn game_setup(commands: &mut Commands, materials: Res<Materials>) {
    commands.spawn(
        SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        }
    ).with(SnakeHead);
}

fn snake_movement(mut snake_head: Query<(&mut SnakeHead, &mut Transform)>) {
    for (_head, mut head_pos) in snake_head.iter_mut() {
        head_pos.translation.x += 1.0;        
    }
}

fn snake_spawn(
    commands: &mut Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: ResMut<SnakeSpawnTimer>,
    mut serial: ResMut<Nexys4Serial>
) {
    timer.0.tick(time.delta_seconds());
    if timer.0.finished() {
        if let Ok(temp) = serial.read_one_byte() {
            println!("receive serial data: {}", temp);
            serial.update_temp(temp);
        }
        let mut transfrom = Transform::default();
        transfrom.translation.y = serial.prev_temp() as f32;
        commands.spawn(
            SpriteBundle {
                material: materials.head_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: transfrom,
                ..Default::default()
            }
        )
        .with(SnakeHead);
    }
}

fn main() {
    println!("hello, nexys4!");
    App::build()
        .add_resource(
            SnakeSpawnTimer(
                Timer::new(Duration::from_millis(100. as u64), true)
            )
        )
        .add_resource(
            Nexys4Serial::first_available(115_200).expect("failed to get serial")
        )
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(game_setup.system()))
        .add_system(snake_movement.system())
        .add_system(snake_spawn.system())
        .add_plugins(DefaultPlugins)
        .run();
}
