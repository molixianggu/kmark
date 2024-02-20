#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

use games::{AssetLoadPlugin, ClientPluginGroup, GameState, PagePlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // 初始化游戏引擎
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen,
                        title: "KM".to_string(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        // 初始化客户端插件
        let client_plugin_group = ClientPluginGroup::new(0);
        app.add_plugins(client_plugin_group.build());

        // 初始化游戏状态
        app.init_state::<GameState>()
            .insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)));

        // 加载资源和页面
        app.add_plugins((AssetLoadPlugin, PagePlugin::new()));

        // 调试插件
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                bevy::diagnostic::FrameTimeDiagnosticsPlugin,
                bevy::diagnostic::LogDiagnosticsPlugin::default(),
            ));
        }
    }
}

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
