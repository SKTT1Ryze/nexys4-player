//! 子弹控制

use bevy::prelude::*;
use super::config::*;

pub struct ButtleBuilder {
    /// Texture 的句柄
    pub texture: Handle<Texture>,
    /// 颜色
    pub color: Color,
    pub size: Vec2,
}

impl ButtleBuilder {
    pub fn type0(texture: Handle<Texture>) -> Self {
        Self {
            texture,
            color: Color::WHITE,
            size: Vec2::new(BULLET_SIZE, BULLET_SIZE),
        }
    }
}