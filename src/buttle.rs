//! 子弹控制

use super::config::*;
use bevy::prelude::*;

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
            color: Color::RED,
            size: Vec2::new(BULLET_SIZE, BULLET_SIZE),
        }
    }

    pub fn type1(texture: Handle<Texture>) -> Self {
        Self {
            texture,
            color: Color::BLUE,
            size: Vec2::new(BULLET_SIZE, BULLET_SIZE),
        }
    }
}

/// 记录子弹的信息
pub struct Buttle {
    /// 所有者
    ///
    /// 目前通过 id 的方式呈现，后面考虑使用引用
    pub owner_id: u32,
    /// 碰撞次数
    collision_times: u32,
    /// 最大碰撞次数
    max_times: u32,
}

impl Buttle {
    pub fn new(owner_id: u32, max_times: u32) -> Self {
        Self {
            owner_id,
            collision_times: 0,
            max_times,
        }
    }

    /// 进行碰撞操作，返回是否该被销毁
    pub fn crash(&mut self) -> bool {
        self.collision_times += 1;
        if self.collision_times >= self.max_times {
            return true;
        } else {
            return false;
        }
    }
}
