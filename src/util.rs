pub struct TextureSize {
    pub columns: usize,
    pub rows: usize,
}

impl TextureSize {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self { columns, rows }
    }
}

/// 判断两个矩形是否相交
pub fn is_intersected(
    left_x: f32,
    right_x: f32,
    top_x: f32,
    bottom_x: f32,
    left_y: f32,
    right_y: f32,
    top_y: f32,
    bottom_y: f32,
) -> bool {
    let left = left_x.max(left_y);
    let bottom = bottom_x.max(bottom_y);
    let right = right_x.min(right_y);
    let top = top_x.min(top_y);
    !(left > right || bottom > top)
}

/// 角色的朝向
#[derive(PartialEq, Eq)]
pub enum TOWARD {
    Up,
    Down,
    Left,
    Right,
}
