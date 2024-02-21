use base64::prelude::*;
use lightyear::connection::netcode::NetcodeServer;
use rocket::serde::json::Json;
use std::net::SocketAddr;
use std::str::FromStr;

use crate::configs;
use crate::model::Response;

#[get("/login")]
fn login() -> Json<Response<String>> {
    let private_key = BASE64_STANDARD
        .decode(configs::CONFIG.private_key.clone())
        .unwrap();

    let bind_addr = "127.0.0.1:5000";
    let mut server =
        NetcodeServer::new(configs::CONFIG.protocol_id, private_key.try_into().unwrap()).unwrap();

    let client_id = 123u64;
    let token = server
        .token(client_id, SocketAddr::from_str(bind_addr).unwrap())
        .expire_seconds(60) // defaults to 30 seconds, negative for no expiry
        .timeout_seconds(-1) // defaults to 15 seconds, negative for no timeout
        .generate()
        .unwrap();

    let result = token.try_into_bytes().unwrap();

    // base64 encode the token
    Response::success(BASE64_STANDARD.encode(&result))
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
