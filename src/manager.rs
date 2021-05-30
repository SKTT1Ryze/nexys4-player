//! 资源管理器

use bevy::prelude::*;

pub struct AssetsManager {
    pub fonts: Vec<Handle<Font>>,
    pub texture: Vec<Handle<Texture>>
}

impl AssetsManager {
    pub fn empty() -> Self {
        Self {
            fonts: Vec::new(),
            texture: Vec::new()
        }
    }
}