#![allow(dead_code)]


use lightyear::connection::netcode::Key;

pub mod message;
pub mod player;
pub mod protocol;

pub use player::PlayerBundle;
pub use protocol::{protocol, shared_config, MyProtocol};

pub const CLIENT_PORT: u16 = 0;
pub const SERVER_PORT: u16 = 5000;
pub const PROTOCOL_ID: u64 = 0;

pub const KEY: Key = [0; 32];

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Transports {
    #[cfg(not(target_family = "wasm"))]
    Udp,
    WebTransport,
    WebSocket,
}
