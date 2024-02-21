#![allow(dead_code)]

pub mod message;
pub mod player;
pub mod protocol;

pub use player::PlayerBundle;
pub use protocol::{protocol, shared_config, MyProtocol};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Transports {
    #[cfg(not(target_family = "wasm"))]
    Udp,
    WebTransport,
    WebSocket,
}
