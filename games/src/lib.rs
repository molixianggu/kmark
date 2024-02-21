mod client;
mod enums;
mod game;
mod loader;
mod protocol;
mod server;
mod resource;

pub use client::ClientPluginGroup;
pub use server::ServerPluginGroup;

pub use enums::{GameState, Inputs};
pub use game::PagePlugin;
pub use loader::AssetLoadPlugin;
pub use protocol::Transports;
