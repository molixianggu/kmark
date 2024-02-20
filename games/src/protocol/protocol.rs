use std::time::Duration;

use lightyear::prelude::*;

use super::message::Messages;
use super::player::{Components, ComponentsKind};
use crate::enums::input::Inputs;

protocolize! {
    Self = MyProtocol,
    Message = Messages,
    Component = Components,
    Input = Inputs,
}

#[derive(Channel)]
pub struct MainChannel;

pub fn protocol() -> MyProtocol {
    let mut protocol = MyProtocol::default();
    protocol.add_channel::<MainChannel>(ChannelSettings {
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
        direction: ChannelDirection::Bidirectional,
        priority: 1.0,
    });
    protocol
}

pub fn shared_config() -> SharedConfig {
    SharedConfig {
        enable_replication: true,
        client_send_interval: Duration::default(),
        server_send_interval: Duration::from_millis(40),
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
    }
}
