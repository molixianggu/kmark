use bevy::prelude::*;
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
