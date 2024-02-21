use std::net::{Ipv4Addr, SocketAddr};

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::utils::Duration;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use crate::protocol::*;

pub struct ServerPluginGroup {
    pub headless: bool,
    pub(crate) lightyear: ServerPlugin<MyProtocol>,
}

impl ServerPluginGroup {
    pub async fn new(port: u16, transport: Transports, headless: bool, private_key: [u8; 32], protocol_id: u64) -> ServerPluginGroup {
        // Step 1: create the io (transport + link conditioner)
        let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);
        let transport_config = match transport {
            Transports::Udp => TransportConfig::UdpSocket(server_addr),
            // if using webtransport, we load the certificate keys
            Transports::WebTransport => {
                let certificate =
                    Certificate::load("certificates/cert.pem", "certificates/key.pem")
                        .await
                        .unwrap();
                let digest = &certificate.hashes()[0];
                info!("Generated self-signed certificate with digest: {}", digest);
                TransportConfig::WebTransportServer {
                    server_addr,
                    certificate,
                }
            }
            Transports::WebSocket => TransportConfig::WebSocketServer { server_addr },
        };
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(200),
            incoming_jitter: Duration::from_millis(20),
            incoming_loss: 0.05,
        };

        // Step 2: define the server configuration
        let config = ServerConfig {
            shared: shared_config().clone(),
            net: NetConfig::Netcode {
                config: NetcodeConfig::default()
                    .with_protocol_id(protocol_id)
                    .with_key(private_key),
                io: IoConfig::from_transport(transport_config).with_conditioner(link_conditioner),
            },
            ..default()
        };

        // Step 3: create the plugin
        let plugin_config = PluginConfig::new(config, protocol());
        ServerPluginGroup {
            headless,
            lightyear: ServerPlugin::new(plugin_config),
        }
    }
}

impl PluginGroup for ServerPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        let builder = PluginGroupBuilder::start::<Self>().add(self.lightyear);
        builder
    }
}


