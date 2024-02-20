use std::net::{Ipv4Addr, SocketAddr};

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::utils::Duration;
use lightyear::prelude::client::*;
use lightyear::prelude::*;

use crate::protocol::{protocol, shared_config, MyProtocol, Transports, KEY, PROTOCOL_ID};

pub struct ClientPluginGroup {
    client_id: ClientId,
    lightyear: ClientPlugin<MyProtocol>,
}

impl ClientPluginGroup {
    pub fn new(client_id: u64) -> ClientPluginGroup {
        let config = ClientConfig {
            shared: shared_config(),
            net: NetConfig::default(),
            interpolation: InterpolationConfig {
                delay: InterpolationDelay::default().with_send_interval_ratio(2.0),
                custom_interpolation_logic: false,
            },
            ..default()
        };
        let plugin_config = PluginConfig::new(config, protocol());
        ClientPluginGroup {
            client_id,
            lightyear: ClientPlugin::new(plugin_config),
        }
    }
}

impl PluginGroup for ClientPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(self.lightyear)
    }
}
