use bevy::prelude::*;
use bevy_entitiles::tiled::resources::TiledTilemapManger;
use lightyear::client::events::{EntitySpawnEvent, InputEvent as ClientInputEvent};
use lightyear::client::input::InputSystemSet;
use lightyear::client::prediction::Predicted;
use lightyear::client::resource::connect_with_token;
use lightyear::server::events::{ConnectEvent, DisconnectEvent, InputEvent as ServerInputEvent};
use std::collections::HashMap;

use super::Page;
use crate::loader::TextureAssets;
use crate::protocol::player::{PlayerAnimation, PlayerAttribute, PlayerId, PlayerPosition};
use crate::protocol::protocol::ClientMut;
use crate::resource::{ClientGlobal, ServerGlobal, TokenResource};
use crate::{
    enums::{GameState, InputDirection, Inputs},
    protocol::PlayerBundle,
};

#[derive(Component)]
pub struct GamePage;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerSelf;

impl GamePage {
    // System

    fn load_map(mut commands: Commands, mut manager: ResMut<TiledTilemapManger>) {
        info!("开始加载地图");
        manager.switch_to(&mut commands, "01".to_string(), None);
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
        player: Query<(Entity, &PlayerId, &PlayerPosition)>,
        global: Res<ClientGlobal>,
        textures: Res<TextureAssets>,
    ) {
        for event in reader.read() {
            if let Ok((entity, player_id, position)) = player.get(event.entity()) {
                info!("创建玩家: {:?}", player_id);
                commands.entity(entity).insert((
                    Player,
                    SpriteBundle {
                        texture: textures.moko.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            position.x, position.y, 0.5,
                        )),
                        ..Default::default()
                    },
                    TextureAtlas::from(textures.moko_layout.clone()),
                ));

                if player_id.is_equal(global.client_id) {
                    commands.entity(entity).insert(PlayerSelf);
                }
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

        if keypress.just_pressed(KeyCode::Space) {
            return client.add_input(Inputs::Jump);
        }

        if keypress.just_pressed(KeyCode::KeyE) {
            return client.add_input(Inputs::Action);
        }

        if keypress.just_pressed(KeyCode::KeyQ) {
            return client.add_input(Inputs::SpeedUp);
        }

        if keypress.just_pressed(KeyCode::KeyZ) {
            return client.add_input(Inputs::SpeedDown);
        }

        client.add_input(Inputs::None)
    }

    fn player_movement(
        mut player_query: Query<
            (
                &mut PlayerPosition,
                &mut PlayerAnimation,
                &mut PlayerAttribute,
            ),
            With<Predicted>,
        >,
        mut input_reader: EventReader<ClientInputEvent<Inputs>>,
        time: Res<Time>,
    ) {
        for input in input_reader.read() {
            if let Some(input) = input.input() {
                for (position, animation, attribute) in player_query.iter_mut() {
                    Self::shared_movement_behaviour(position, animation, attribute, input, &time);
                }
            }
        }
    }

    fn draw_boxes(mut gizmos: Gizmos, players: Query<&PlayerPosition>) {
        for position in &players {
            gizmos.rect(
                Vec3::new(position.x, position.y, 0.0),
                Quat::IDENTITY,
                Vec2::ONE * 50.0,
                Color::RED,
            );
        }
    }

    fn sync_player(
        mut players: Query<(
            &PlayerPosition,
            &mut Transform,
            &PlayerAnimation,
            &mut TextureAtlas,
        )>,
    ) {
        for (position, mut transform, animation, mut atlas) in players.iter_mut() {
            transform.translation = Vec3::new(position.x, position.y, transform.translation.z);
            atlas.index = animation.frame as usize;
        }
    }

    fn sync_camera(
        mut camera_query: Query<&mut Transform, With<Camera>>,
        players: Query<&PlayerPosition, With<PlayerSelf>>,
    ) {
        for mut transform in camera_query.iter_mut() {
            if let Ok(position) = players.get_single() {
                transform.translation = Vec3::new(position.x, position.y, 10.0);
            }
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
            let entity = commands.spawn(PlayerBundle::new(*client_id, Vec2::new(10.0, 10.0)));
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
        mut player_query: Query<(
            &mut PlayerPosition,
            &mut PlayerAnimation,
            &mut PlayerAttribute,
        )>,
        mut input_reader: EventReader<ServerInputEvent<Inputs>>,
        global: Res<ServerGlobal>,
        time: Res<Time>,
    ) {
        for input in input_reader.read() {
            let client_id = input.context();
            if let Some(input) = input.input() {
                if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                    if let Ok((position, animation, attribute)) =
                        player_query.get_mut(*player_entity)
                    {
                        Self::shared_movement_behaviour(
                            position, animation, attribute, input, &time,
                        );
                    }
                }
            }
        }
    }

    fn shared_movement_behaviour(
        mut position: Mut<PlayerPosition>,
        mut animation: Mut<PlayerAnimation>,
        mut attribute: Mut<PlayerAttribute>,
        input: &Inputs,
        time: &Time,
    ) {
        let mut direction_i = u32::MAX;
        match input {
            Inputs::Direction(direction) => {
                if direction.up {
                    position.y += attribute.speed * 0.1;
                    direction_i = 3;
                }
                if direction.down {
                    position.y -= attribute.speed * 0.1;
                    direction_i = 0;
                }
                if direction.left {
                    position.x -= attribute.speed * 0.1;
                    direction_i = 1;
                }
                if direction.right {
                    position.x += attribute.speed * 0.1;
                    direction_i = 2;
                }
            }
            Inputs::SpeedUp => attribute.speed += 1.0,
            Inputs::SpeedDown => attribute.speed -= 1.0,
            _ => {}
        }
        let v = if direction_i != u32::MAX {
            direction_i * 3
                + if (time.elapsed().as_secs_f32() * attribute.speed * 0.5).sin() < 0.0 {
                    0
                } else {
                    2
                }
        } else {
            animation.frame - (animation.frame % 3) + 1
        };
        if v != animation.frame {
            animation.frame = v;
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

        app.register_type::<PlayerPosition>();
        app.register_type::<PlayerAnimation>();
        app.register_type::<PlayerAttribute>();

        app.add_systems(OnEnter(Self::state()), (Self::connection,))
            .add_systems(
                Update,
                (Self::create_player, Self::sync_camera).run_if(in_state(Self::state())),
            )
            .add_systems(
                FixedPreUpdate,
                Self::buffer_input
                    .in_set(InputSystemSet::BufferInputs)
                    .run_if(in_state(Self::state())),
            )
            .add_systems(
                FixedUpdate,
                Self::player_movement.run_if(in_state(Self::state())),
            )
            .add_systems(
                FixedPostUpdate,
                Self::sync_player.run_if(in_state(Self::state())),
            );
    }

    fn server_setup(app: &mut App) {
        app.insert_resource(ServerGlobal {
            client_id_to_entity_id: HashMap::new(),
        })
        .add_systems(FixedUpdate, (Self::movement,))
        .add_systems(Update, (Self::handle_connections,));
    }

    fn build(app: &mut App) {
        app.add_plugins(super::tiles::TilesPlugin);
        app.add_systems(OnEnter(Self::state()), (Self::load_map,));
    }
}
