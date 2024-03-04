use base64::prelude::*;
use lightyear::connection::netcode::NetcodeServer;
use rand::Rng;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::str::FromStr;

use crate::configs;
use crate::model::Response;


#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
    client_id: u64,
    name: String,
}


#[get("/login")]
fn login() -> Json<Response<LoginResponse>> {
    let private_key = BASE64_STANDARD
        .decode(configs::CONFIG.private_key.clone())
        .unwrap();

    let bind_addr = "127.0.0.1:5000";
    let mut server =
        NetcodeServer::new(configs::CONFIG.protocol_id, private_key.try_into().unwrap()).unwrap();

    let mut rng = rand::thread_rng();

    let client_id = rng.gen::<u64>();
    let token = server
        .token(client_id, SocketAddr::from_str(bind_addr).unwrap())
        .expire_seconds(60) // defaults to 30 seconds, negative for no expiry
        .timeout_seconds(-1) // defaults to 15 seconds, negative for no timeout
        .generate()
        .unwrap();

    let result = token.try_into_bytes().unwrap();

    // base64 encode the token
    Response::success(LoginResponse{
        token: BASE64_STANDARD.encode(&result),
        client_id,
        name: "test".to_string(),
    })
}

pub fn users_routes() -> Vec<rocket::Route> {
    let routes = routes![login];
    routes
        .iter()
        .map(|r| {
            r.clone()
                .map_base(|base| format!("/users{}", base))
                .unwrap()
        })
        .collect()
}
