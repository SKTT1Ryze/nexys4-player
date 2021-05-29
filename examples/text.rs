use std::ops::AddAssign;
use std::time;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
type Result<T> = std::result::Result<T, TextError>;

/// 本例子展示如何往系统添加文字并且如何更新它们
/// 本例子显示当前的 FPS 在窗口左上角
fn main() {
    App::build()
        .add_resource(TextSpawnTimer(
            Timer::new(time::Duration::from_secs(10. as u64), true)
        ))
        .add_resource(TextManager::empty())
        .add_resource(AssetManager::empty())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(text_update_system.system())
        .add_system(text_spawn.system())
        .run();
}

#[derive(Debug)]
enum TextError {
    NoSource
}

/// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

/// 文本产生定时器
struct TextSpawnTimer(Timer);

/// 文本管理器
struct TextManager {
    /// 需要生成的文本
    pub source: Vec<String>,
    /// 下一个文本的位置
    position: Rect<Val>
}

impl TextManager {
    pub fn empty() -> Self {
        Self {
            source: Vec::new(),
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            }
        }
    }

    pub fn spawn(&mut self, font: Handle<Font>) -> Result<TextBundle> {
        if let Some(text) = self.source.pop() {
            let ret_text = Text {
                value: text,
                font,
                style: TextStyle {
                    font_size: 25.0,
                    color: Color::RED,
                    ..Default::default()
                }
            };
            let ret_pos = self.position;
            self.update_next_pos();
            Ok(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position: ret_pos,
                    ..Default::default()
                },
                text: ret_text,
                ..Default::default()
            })
        } else {
            Err(TextError::NoSource)
        }
    }
    
    /// 更新下一个文本生成的位置
    pub fn update_next_pos(&mut self) {
        match self.position.left {
            Val::Px(x) => {
                if x > 500.0 {
                    self.position.left = Val::Px(0.0);
                    self.position.top.add_assign(10.0);
                }
            },
            _ => panic!("unsupport position value")
        }
    }
}

/// 资源管理器
struct AssetManager {
    fonts: Vec<Handle<Font>>
}

impl AssetManager {
    pub fn empty() -> Self {
        Self {
            fonts: Vec::new()
        }
    }
    pub fn add_font(&mut self, font: Handle<Font>) {
        self.fonts.push(font);
    }
    pub fn current_font(&self) -> Option<Handle<Font>> {
        if let Some(font) = self.fonts.get(0) {
            Some(font.clone())
        } else {
            None
        }
    }
}


fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

// 不断产生文本
fn text_spawn(
    commands: &mut Commands,
    time: Res<Time>,
    mut timer: ResMut<TextSpawnTimer>,
    mut text_manager: ResMut<TextManager>,
    asset_manager: Res<AssetManager>
) {
    timer.0.tick(time.delta_seconds());
    if timer.0.finished() {
        let font = asset_manager.current_font().expect("no available font");
        if let Ok(text) = text_manager.spawn(font) {
            commands.spawn(text);
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut text_manager: ResMut<TextManager>,
    mut asset_manager: ResMut<AssetManager>
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: font.clone(),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(FpsText);
    asset_manager.add_font(font);
    text_manager.source.push("Hello".to_string());
    text_manager.source.push("Bevy".to_string());
}