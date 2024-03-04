use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32., tile_size_y = 32., columns = 3, rows = 12))]
    pub moko_layout: Handle<TextureAtlasLayout>,

    #[asset(image(sampler = nearest))]
    #[asset(path = "textures/player/moko.png")]
    pub moko: Handle<Image>,
}
