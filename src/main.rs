// 华中科技大学接口技术大作业
use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

const pos_vec: [(i32, i32); 5] = [(0, 0), (10, 10), (20, 20), (30, 30), (40, 40)];
static mut pos_idx: usize = 0;
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
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(
        Materials {
            head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into())
        }
    );
}

fn game_setup(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn(
        SpriteComponents {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        }
    ).with(SnakeHead);
}

fn snake_movement(mut head_positions: Query<With<SnakeHead, &mut Transform>>) {
    for mut transform in &mut head_positions.iter() {
        let x = transform.translation().x();
        transform.translation_mut().set_x( x + 2.0);        
    }
}

fn snake_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: ResMut<SnakeSpawnTimer>
) {
    timer.0.tick(time.delta_seconds);
    let mut transfrom = Transform::default();
    let mut vec_3 = Vec3::default();
    vec_3.set_x(pos_vec[unsafe{pos_idx}].0 as f32);
    vec_3.set_y(pos_vec[unsafe{pos_idx}].1 as f32);
    transfrom.set_translation(vec_3);
    if timer.0.finished {
        commands.spawn(
            SpriteComponents {
                material: materials.head_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: transfrom,
                ..Default::default()
            }
        )
        .with(SnakeHead);
        unsafe { pos_idx = (pos_idx + 1) % 5; }
    }
}

fn main() {
    println!("hello, nexys4!");
    App::build()
        .add_resource(SnakeSpawnTimer(Timer::new(
            Duration::from_millis(500. as u64),
            true,
        )))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", game_setup.system())
        .add_system(snake_movement.system())
        .add_system(snake_spawn.system())
        .add_default_plugins()
        .run();
}
