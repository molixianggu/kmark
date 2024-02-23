use bevy::prelude::*;
use lightyear::client::events::EntitySpawnEvent;
use lightyear::client::resource::connect_with_token;
use lightyear::server::events::{ConnectEvent, DisconnectEvent, InputEvent};
use lightyear::shared::tick_manager::TickManager;
use std::collections::HashMap;

use super::Page;
use crate::protocol::player::{PlayerColor, PlayerId, PlayerPosition};
use crate::protocol::protocol::ClientMut;
use crate::resource::{ClientGlobal, ServerGlobal, TokenResource};
use crate::{enums::{GameState, InputDirection, Inputs}, protocol::PlayerBundle};

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

    fn create_player(
        mut commands: Commands,
        mut reader: EventReader<EntitySpawnEvent>,
        player: Query<(Entity, &PlayerId, &PlayerPosition, &PlayerColor)>,
    ) {
        for event in reader.read() {
            if let Ok(player) = player.get(event.entity()) {
                // commands.entity(entity);
                info!("创建玩家: {:?}", player.1);
            }
        }
    }

    fn buffer_input(mut client: ClientMut, keypress: Res<ButtonInput<KeyCode>>) {
        let mut direction = InputDirection {
            up: false,
            down: false,
            left: false,
            right: false,
        };
        if keypress.pressed(KeyCode::KeyW) || keypress.pressed(KeyCode::ArrowUp) {
            direction.up = true;
        }
        if keypress.pressed(KeyCode::KeyS) || keypress.pressed(KeyCode::ArrowDown) {
            direction.down = true;
        }
        if keypress.pressed(KeyCode::KeyA) || keypress.pressed(KeyCode::ArrowLeft) {
            direction.left = true;
        }
        if keypress.pressed(KeyCode::KeyD) || keypress.pressed(KeyCode::ArrowRight) {
            direction.right = true;
        }
        if !direction.is_none() {
            return client.add_input(Inputs::Direction(direction));
        }
        // info!("Sending input: {:?} on tick: {:?}", &input, client.tick());
        return client.add_input(Inputs::None);
    }

    fn draw_boxes(mut gizmos: Gizmos, players: Query<(&PlayerPosition, &PlayerColor)>) {
        for (position, color) in &players {
            gizmos.rect(
                Vec3::new(position.x, position.y, 0.0),
                Quat::IDENTITY,
                Vec2::ONE * 50.0,
                color.0,
            );
        }
    }

    /**
     * 服务器系统
     */
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

    fn movement(
        mut position_query: Query<&mut PlayerPosition>,
        mut input_reader: EventReader<InputEvent<Inputs>>,
        global: Res<ServerGlobal>,
        tick_manager: Res<TickManager>,
    ) {
        for input in input_reader.read() {
            let client_id = input.context();
            if let Some(input) = input.input() {
                debug!(
                    "Receiving input: {:?} from client: {:?} on tick: {:?}",
                    input,
                    client_id,
                    tick_manager.tick()
                );
                if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                    if let Ok(position) = position_query.get_mut(*player_entity) {
                        Self::shared_movement_behaviour(position, input);
                    }
                }
            }
        }
    }

    fn shared_movement_behaviour(mut position: Mut<PlayerPosition>, input: &Inputs) {
        const MOVE_SPEED: f32 = 10.0;
        match input {
            Inputs::Direction(direction) => {
                if direction.up {
                    position.y += MOVE_SPEED;
                }
                if direction.down {
                    position.y -= MOVE_SPEED;
                }
                if direction.left {
                    position.x -= MOVE_SPEED;
                }
                if direction.right {
                    position.x += MOVE_SPEED;
                }
            }
            _ => {}
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
        app.add_systems(OnEnter(Self::state()), (Self::connection,))
            .add_systems(Update, (Self::create_player, Self::draw_boxes, Self::buffer_input));
    }

    fn server_setup(app: &mut App) {
        app.insert_resource(ServerGlobal {
            client_id_to_entity_id: HashMap::new(),
        })
        .add_systems(Update, (Self::handle_connections, Self::movement));
    }

    fn build(app: &mut App) {
        app.add_systems(OnEnter(Self::state()), (Self::load_map,));
    }
}
