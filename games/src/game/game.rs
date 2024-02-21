use bevy::prelude::*;
use lightyear::client::resource::connect_with_token;
use lightyear::connection::netcode::ConnectToken;
use lightyear::server::events::{ConnectEvent, DisconnectEvent};
use std::collections::HashMap;

use super::Page;
use crate::{enums::GameState, protocol::PlayerBundle};
use crate::resource::{ClientGlobal, ServerGlobal};

#[derive(Component)]
pub struct GamePage;

impl GamePage {
    // System

    fn load_map() {
        info!("开始加载地图");
    }

    fn connection(world: &mut World) {
        let token = world.get_resource::<TokenResource>().unwrap();
        if let Ok(_) = connect_with_token(world, token.value.clone()) {
            info!("连接成功");
        }
    }

    fn handle_connections(
        mut commands: Commands,
        mut connections: EventReader<ConnectEvent>,
        mut disconnections: EventReader<DisconnectEvent>,
        mut global: ResMut<ServerGlobal>,
    ) {
        for connection in connections.read() {
            let client_id = connection.context();
            info!("玩家 {} 连接", client_id);
            let entity = commands.spawn(PlayerBundle::new(
                *client_id,
                Vec2::new(10.0, 10.0),
                Color::RED,
            ));
            global
                .client_id_to_entity_id
                .insert(*client_id, entity.id());
        }
        for disconnection in disconnections.read() {
            info!("玩家 {} 断开连接", disconnection.context());
            let client_id = disconnection.context();
            if let Some(entity) = global.client_id_to_entity_id.remove(client_id) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

impl Page for GamePage {
    type SelfType = Self;

    fn name() -> &'static str {
        "game"
    }

    fn state() -> GameState {
        GameState::Game
    }

    fn client_setup(app: &mut App) {
        app.insert_resource(ClientGlobal { client_id: 0 });
        app.add_systems(OnEnter(Self::state()), (Self::connection, ));
    }

    fn server_setup(app: &mut App) {
        app.insert_resource(ServerGlobal {
            client_id_to_entity_id: HashMap::new(),
        })
        .add_systems(Update, (Self::handle_connections,));
    }

    fn build(app: &mut App) {
        app.add_systems(OnEnter(Self::state()), (Self::load_map,));
    }
}

#[derive(Resource)]
pub(crate) struct TokenResource {
    pub value: ConnectToken,
}
