mod client;
mod enums;
mod game;
mod loader;
mod protocol;
mod server;

pub use client::ClientPluginGroup;
pub use server::ServerPluginGroup;

pub use enums::{GameState, Inputs};
pub use game::PagePlugin;
pub use loader::AssetLoadPlugin;
