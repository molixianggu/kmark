use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use lightyear::prelude::client::*;

use crate::protocol::{protocol, shared_config, MyProtocol};

pub struct ClientPluginGroup {
    lightyear: ClientPlugin<MyProtocol>,
}

impl ClientPluginGroup {
    pub fn new() -> ClientPluginGroup {
        let config = ClientConfig {
            shared: shared_config(),
            net: NetConfig::default(),
            interpolation: InterpolationConfig {
                delay: InterpolationDelay::default().with_send_interval_ratio(2.0),
                custom_interpolation_logic: false,
            },
            sync: SyncConfig::default(),
            ..default()
        };
        let plugin_config = PluginConfig::new(config, protocol());
        ClientPluginGroup {
            lightyear: ClientPlugin::new(plugin_config),
        }
    }
}

impl PluginGroup for ClientPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(self.lightyear)
    }
}
