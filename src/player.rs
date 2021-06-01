//! 角色控制

use bevy::prelude::*;
use super::config::*;
use super::util::*;

/// 玩家控制的角色构建器
pub struct PlayerBuilder {
    /// Texture 的句柄
    pub texture: (Handle<Texture>, TextureSize),
    pub size: Vec2,
    pub transform: Transform
}

impl PlayerBuilder {
    pub fn default0(texture: Handle<Texture>, texture_size: TextureSize) -> Self {
        Self {
            texture: (texture, texture_size),
            size: Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            transform: Transform {
                translation: Vec3 {x: -100.0, y: -100.0, z: 0.0},
                scale: Vec3 {x: SPRITE_SCALE, y: SPRITE_SCALE, z:0.0},
                ..Default::default()
            }
        }
    }

    pub fn default1(texture: Handle<Texture>, texture_size: TextureSize) -> Self {
        Self {
            texture: (texture, texture_size),
            size: Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            transform: Transform {
                translation: Vec3 {x: 100.0, y: 100.0, z: 0.0},
                scale: Vec3 {x: SPRITE_SCALE, y: SPRITE_SCALE, z:0.0},
                ..Default::default()
            }
        }
    }
}

/// 记录角色的一些信息
pub struct Player {
    /// 朝向
    pub toward: TOWARD,
    /// 弹量
    pub bomb: u32,
    /// 得分
    pub score: u32
}

impl std::default::Default for Player {
    fn default() -> Self {
        Self {
            toward: TOWARD::Down,
            bomb: 5,
            score: 0
        }
    }
}

impl Player {
    // 碰撞
    pub fn crash(&mut self) {
        // todo   
    }
}
