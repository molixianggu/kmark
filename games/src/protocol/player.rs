use bevy::{prelude::{default, Bundle, Component, Deref, DerefMut, Entity, EntityMapper, Vec2}, reflect::Reflect};
use derive_more::{Add, Mul};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Mul;

use super::protocol::{MyProtocol, Replicate};

// Player
#[derive(Bundle)]
pub struct PlayerBundle {
    id: PlayerId,
    position: PlayerPosition,
    animation: PlayerAnimation,
    attribute: PlayerAttribute,
    replicate: Replicate,
}

impl PlayerBundle {
    pub fn new(id: ClientId, position: Vec2) -> Self {
        Self {
            id: PlayerId(id),
            position: PlayerPosition(position),
            animation: PlayerAnimation { frame: 1 },
            attribute: PlayerAttribute {
                speed: 10.0,
                skin: "moko".to_string(),
                hp: 100.0,
            },
            replicate: Replicate {
                // prediction_target: NetworkTarget::None,
                prediction_target: NetworkTarget::Only(vec![id]),
                interpolation_target: NetworkTarget::AllExcept(vec![id]),
                ..default()
            },
        }
    }
}

#[derive(Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(ClientId);

impl PlayerId {
    pub fn is_equal(&self, id: u64) -> bool {
        self.0 == id
    }
}

#[derive(
    Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul, Reflect
)]
pub struct PlayerPosition(Vec2);

impl Mul<f32> for &PlayerPosition {
    type Output = PlayerPosition;

    fn mul(self, rhs: f32) -> Self::Output {
        PlayerPosition(self.0 * rhs)
    }
}

#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerAnimation {
    pub frame: u32,
}

#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerAttribute {
    pub speed: f32,
    pub skin: String,
    // pub jump: f32,
    // pub color: Color,
    pub hp: f32,
}

#[derive(Component, Message, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[message(custom_map)]
pub struct PlayerParent(Entity);

impl LightyearMapEntities for PlayerParent {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.0 = entity_mapper.map_entity(self.0);
    }
}

#[component_protocol(protocol = "MyProtocol")]
pub enum Components {
    #[sync(once)]
    PlayerId(PlayerId),
    #[sync(full)]
    PlayerPosition(PlayerPosition),
    #[sync(simple)]
    PlayerAnimation(PlayerAnimation),
    #[sync(once)]
    PlayerAttribute(PlayerAttribute),
}
