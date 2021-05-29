//! 角色控制

use bevy::prelude::*;

/// 玩家控制的角色构建器
pub struct PlayerBuilder {
    /// Texture 的句柄
    pub texture: (Handle<Texture>, TextureSize),
    pub tile_size: Vec2,
    pub transform: Transform
}

impl PlayerBuilder {
    pub fn default0(texture: Handle<Texture>, texture_size: TextureSize) -> Self {
        Self {
            texture: (texture, texture_size),
            tile_size: Vec2::new(10.0, 10.0),
            transform: Transform {
                translation: Vec3 {x: -100.0, y: -100.0, z: 0.0},
                scale: Vec3 {x: 10.0, y: 10.0, z:0.0},
                ..Default::default()
            }
        }
    }

    pub fn default1(texture: Handle<Texture>, texture_size: TextureSize) -> Self {
        Self {
            texture: (texture, texture_size),
            tile_size: Vec2::new(10.0, 10.0),
            transform: Transform {
                translation: Vec3 {x: 100.0, y: 100.0, z: 0.0},
                scale: Vec3 {x: 10.0, y: 10.0, z:0.0},
                ..Default::default()
            }
        }
    }
}

pub struct TextureSize {
    pub columns: usize,
    pub rows: usize
}

impl TextureSize {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns,
            rows
        }
    }
}