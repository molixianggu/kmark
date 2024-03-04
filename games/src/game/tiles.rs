use bevy::prelude::*;
use bevy_entitiles::{
    tiled::{self, resources::TiledLoadConfig},
    tilemap::EntiTilesTilemapPlugin,
    EntiTilesPlugin,
};
use bevy_xpbd_2d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "client")]
        app.add_plugins((
            EntiTilesPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ));

        #[cfg(feature = "server")]
        app.add_plugins((
            // tiled::EntiTilesTiledPlugin,
            PhysicsPlugins::default(),
        ));

        app.insert_resource(TiledLoadConfig {
            map_path: vec!["client/assets/tiled/tmx/01.tmx".to_string()],
            ignore_unregisterd_objects: true,
            z_index: 0.,
        });
    }
}
