// 一些配置参数在这里定义

/// 时钟间隔
pub const TIMER_INTERVAL: f32 = 0.1;

pub const SPRITE_SCALE: f32 = 5.0;

pub const PLAYER_SIZE: f32 = 10.0;

pub const ENEMY_SIZE: f32 = 8.0;

pub const BULLET_SIZE: f32 = 5.0;

/// 地图格子宽度
pub const MAP_BLOCK_WIDTH: f32 = 10.0;

/// 重力加速度
pub const GRAVITY: f32 = -9.821 * 1.0;

/// 敌人 y 坐标对于 x 轴的向下偏移量
pub const ENEMY_OFFSET: f32 = 25.;

/// 串口波特率
pub const SERIAL_BAUD_RATE: u32 = 115_200;

pub const BTN_U: u8 = 'W' as u8;
pub const BTN_D: u8 = 'S' as u8;
pub const BTN_L: u8 = 'A' as u8;
pub const BTN_R: u8 = 'D' as u8;
pub const BTN_C: u8 = 'J' as u8;
