//! 资源管理器

use bevy::prelude::*;
use std::collections::HashMap;

pub struct AssetsManager {
    pub fonts: Vec<Handle<Font>>,
    pub textures: HashMap<String, Handle<Texture>>,
    pub materials: HashMap<String, Handle<ColorMaterial>>
}

impl AssetsManager {
    pub fn empty() -> Self {
        Self {
            fonts: Vec::new(),
            textures: HashMap::new(),
            materials: HashMap::new()
        }
    }
}