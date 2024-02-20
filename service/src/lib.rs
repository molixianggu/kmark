use std::net::{Ipv4Addr, SocketAddr};

use async_compat::Compat;
use bevy::{log::LogPlugin, prelude::*, tasks::IoTaskPool};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);
        app.add_plugins(LogPlugin::default());
        let server_plugin_group = IoTaskPool::get()
            .scope(|s| {
                s.spawn(Compat::new(async {
                    server::ServerPluginGroup::new(
                        protocol::SERVER_PORT,
                        protocol::Transports::WebTransport,
                        true,
                    )
                    .await
                }));
            })
            .pop()
            .unwrap();
        app.add_plugins(server_plugin_group.build());
        app.init_state::<enums::GameState>();
        app.add_plugins(game::PagePlugin::new());
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                bevy::diagnostic::FrameTimeDiagnosticsPlugin,
                bevy::diagnostic::LogDiagnosticsPlugin::default(),
            ));
        }
    }
}
