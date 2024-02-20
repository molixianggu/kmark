use crate::enums::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod font;

pub use font::FontAssets;

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), (setup,))
            .add_systems(OnExit(GameState::Loading), (done,))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Title)
                    .load_collection::<font::FontAssets>(),
            );
    }
}

fn setup(mut _commands: Commands) {
    info!("开始加载资源");
}

fn done(mut _commands: Commands) {
    info!("资源加载完成");
}
