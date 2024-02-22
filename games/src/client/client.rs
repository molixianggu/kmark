use std::net::SocketAddrV4;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use lightyear::{
    prelude::client::*,
    transport::io::{IoConfig, TransportConfig},
};

use crate::protocol::{protocol, shared_config, MyProtocol};

pub struct ClientPluginGroup {
    lightyear: ClientPlugin<MyProtocol>,
}

impl ClientPluginGroup {
    pub fn new() -> ClientPluginGroup {
        let config = ClientConfig {
            shared: shared_config(),
            net: NetConfig::Netcode {
                auth: Authentication::default(),
                config: NetcodeConfig::default(),
                io: IoConfig::from_transport(TransportConfig::WebTransportClient {
                    client_addr: SocketAddrV4::new("127.0.0.1".parse().unwrap(), 0).into(),
                    server_addr: SocketAddrV4::new("127.0.0.1".parse().unwrap(), 5000).into(),
                    #[cfg(target_family = "wasm")]
                    certificate_digest: String::form("e4:b9:7d:03:70:0a:be:75:4c:05:c4:9c:6a:3e:7f:dd:8a:3b:7f:9e:4f:e5:17:d9:53:23:3b:7a:53:b7:37:b3").replace(":", ""),
                }),
            },
            interpolation: InterpolationConfig {
                delay: InterpolationDelay::default().with_send_interval_ratio(2.0),
                custom_interpolation_logic: false,
            },
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
