use bevy::prelude::*;

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
        #[cfg(feature = "server")]
        {
            Self::server_setup(app);
        }
        #[cfg(feature = "client")]
        {
            Self::client_setup(app);
        }

        Self::build(app);
        app
            // 离开页面时，执行 teardown 方法
            .add_systems(OnExit(Self::state()), (Self::SelfType::teardown,));
    }
}
