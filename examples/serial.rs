use bevy::prelude::*;
use std::time::Duration;

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
fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(
        Materials {
            head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into())
        }
    );
}

fn game_setup(commands: &mut Commands, materials: Res<Materials>) {
    commands.spawn(
        SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        }
    ).with(SnakeHead);
}

fn snake_movement(mut snake_head: Query<(&mut SnakeHead, &mut Transform)>) {
    for (_head, mut head_pos) in snake_head.iter_mut() {
        head_pos.translation.x += 1.0;        
    }
}

fn snake_spawn(
    commands: &mut Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: ResMut<SnakeSpawnTimer>,
    mut serial: ResMut<Nexys4Serial>
) {
    timer.0.tick(time.delta_seconds());
    if timer.0.finished() {
        if let Ok(temp) = serial.read_one_byte() {
            println!("receive serial data: {}", temp);
            serial.update_temp(temp);
        }
        let mut transfrom = Transform::default();
        transfrom.translation.y = serial.prev_temp() as f32;
        commands.spawn(
            SpriteBundle {
                material: materials.head_material.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                transform: transfrom,
                ..Default::default()
            }
        )
        .with(SnakeHead);
    }
}

fn main() {
    println!("hello, nexys4!");
    App::build()
        .add_resource(
            SnakeSpawnTimer(
                Timer::new(Duration::from_millis(100. as u64), true)
            )
        )
        .add_resource(
            Nexys4Serial::first_available(115_200).expect("failed to get serial")
        )
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(game_setup.system()))
        .add_system(snake_movement.system())
        .add_system(snake_spawn.system())
        .add_plugins(DefaultPlugins)
        .run();
}

use serialport::SerialPort;
use serialport::Result;

/// 串口抽象
pub struct Nexys4Serial {
    pub port: Box<dyn SerialPort>,
    previous_temp: u8
}

unsafe impl Sync for Nexys4Serial {}

impl Nexys4Serial {
    /// 第一个可用的串口
    pub fn first_available(baud_rate: u32) -> Result<Self> {
        let ports = serialport::available_ports()?;
        let port = serialport::new(&ports[0].port_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()?;
        Ok(
            Nexys4Serial {
                port,
                previous_temp: 0
            }
        )
    }

    /// 获得波特率
    pub fn baud_rate(&self) -> u32 {
        self.port.baud_rate().expect("failed to get baud rate")
    }

    /// 设置波特率
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.port.set_baud_rate(baud_rate).expect("failed to set baud rate");
    }

    /// 读取一个字节
    /// 
    pub fn read_one_byte(&mut self) -> Result<u8> {
        let mut buf = [0u8];
        self.port.read(&mut buf)?;
        Ok(buf[0])
    }

    /// 写一个字节
    /// 
    pub fn write_one_byte(&mut self, byte: u8) -> Result<()> {
        let buf = [byte];
        self.port.write(&buf)?;
        Ok(())
    }


    /// 读取串口数据到缓冲区
    /// 返回读取的字节数
    pub fn read_to_buf(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.port.read(buf)
    }

    /// 从缓冲区中写数据到串口
    pub fn write_to_buf(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.port.write(buf)?;
        Ok(())
    }

    /// 更新温度值
    pub fn update_temp(&mut self, temp: u8) {
        self.previous_temp = temp;
    }

    /// 获得上一次温度值
    pub fn prev_temp(&self) -> u8 {
        self.previous_temp
    }
}