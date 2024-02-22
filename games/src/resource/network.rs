use base64::prelude::*;
use bevy::prelude::*;
use lightyear::connection::netcode::ConnectToken;
use lightyear::prelude::*;

use std::collections::HashMap;

#[derive(Resource)]
pub struct ServerGlobal {
    pub client_id_to_entity_id: HashMap<ClientId, Entity>,
}

#[derive(Resource)]
pub struct ClientGlobal {
    pub client_id: ClientId,
}

#[derive(Resource)]
pub struct TokenResource {
    pub value: ConnectToken,
}

impl TokenResource {
    pub fn new(value: String) -> Self {
        let token_bytes = BASE64_STANDARD.decode(value).unwrap();
        let client = ConnectToken::try_from_bytes(&token_bytes).unwrap();
        Self { value: client }
    }
}
