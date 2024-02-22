use async_compat::Compat;
use bevy::{log::LogPlugin, prelude::*, tasks::IoTaskPool};
use games::{GameState, PagePlugin, ServerPluginGroup, Transports};
use base64::prelude::*;

pub struct GamePlugin;

mod configs;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);
        app.add_plugins(LogPlugin::default());
        let server_plugin_group = IoTaskPool::get()
            .scope(|s| {
                s.spawn(Compat::new(async {
                    ServerPluginGroup::new(
                        5000, 
                        Transports::WebTransport, 
                        true, 
                        BASE64_STANDARD.decode(configs::CONFIG.private_key.clone()).unwrap().try_into().unwrap(), 
                        configs::CONFIG.protocol_id,
                    ).await
                }));
            })
            .pop()
            .unwrap();
        app.add_plugins(server_plugin_group.build());
        app.init_state::<GameState>();
        app.add_plugins(PagePlugin::new());
        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((
        //         bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        //         bevy::diagnostic::LogDiagnosticsPlugin::default(),
        //     ));
        // }
    }
}

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
