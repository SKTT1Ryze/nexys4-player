//! 敌人控制

use super::config::*;
use super::util::*;
use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

/// 敌人构建器
#[allow(unused)]
pub struct EnemyBuilder {
    /// Texture 的句柄
    pub texture: (Handle<Texture>, TextureSize),
    pub size: Vec2,
    pub transform: Transform,
}

impl EnemyBuilder {
    #[allow(unused)]
    pub fn spawn(
        wins: &Windows,
        texture: Handle<Texture>,
        texture_size: TextureSize,
        rand_seed: &mut ThreadRng,
    ) -> Self {
        let win = wins.get_primary().unwrap();
        let win_left = -(win.width() / 2.);
        let win_right = win.width() / 2.;
        let mut transform = Transform::default();
        transform.translation.y -= ENEMY_OFFSET;
        transform.translation.x = rand_seed.gen_range(win_left, win_right);

        Self {
            texture: (texture, texture_size),
            size: Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
            transform,
        }
    }
}

/// 记录敌人的信息
#[allow(unused)]
pub struct Enemy {
    /// 血量
    pub hp: u32,
    /// 朝向
    pub toward: TOWARD,
}

impl Enemy {}
