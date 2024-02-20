#[macro_use]
extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, m())
}

fn m() -> String {
    use base64::prelude::*;
    use lightyear::connection::netcode::{generate_key, NetcodeServer};
    use std::net::SocketAddr;
    use std::str::FromStr;

    let private_key = generate_key();
    let protocol_id = 1;
    let bind_addr = "0.0.0.0:0";
    let mut server = NetcodeServer::new(protocol_id, private_key).unwrap();

    let client_id = 123u64;
    let token = server
        .token(client_id, SocketAddr::from_str(bind_addr).unwrap())
        .expire_seconds(60) // defaults to 30 seconds, negative for no expiry
        .timeout_seconds(-1) // defaults to 15 seconds, negative for no timeout
        .generate()
        .unwrap();

    let result = token.try_into_bytes().unwrap();

    // base64 encode the token
    BASE64_STANDARD.encode(&result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello])
}
