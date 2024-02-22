use bevy::{prelude::*, utils::info};

use crate::enums::GameState;

mod game;
mod title;

pub struct PagePlugin {}

impl PagePlugin {
    pub fn new() -> Self {
        Self {}
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "client")]
        {
            app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
        }

        app.add_systems(Startup, init);

        title::TitlePage::register(app);
        game::GamePage::register(app);
    }
}

pub trait Page {
    type SelfType: 'static + Component + Page;

    fn name() -> &'static str;
    fn state() -> GameState;

    fn build(app: &mut App);

    fn client_setup(_: &mut App) {}

    fn server_setup(_: &mut App) {}

    fn teardown(mut commands: Commands, query: Query<Entity, With<Self::SelfType>>) {
        for entity in &mut query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        debug!("{} 清理", Self::name());
    }

    fn register(app: &mut App) {
        info!("注册 {}", Self::name());
        #[cfg(feature = "server")]
        {
            Self::server_setup(app);
            info("server");
        }
        #[cfg(feature = "client")]
        {
            Self::client_setup(app);
            info("client");
        }

        Self::build(app);
        app
            // 离开页面时，执行 teardown 方法
            .add_systems(OnExit(Self::state()), (Self::SelfType::teardown,));
    }
}
