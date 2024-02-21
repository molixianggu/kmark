use std::vec;

#[macro_use]
extern crate rocket;

mod api;
mod model;
mod configs;

#[launch]
fn rocket() -> _ {
    let mut routes = vec![];
    routes.extend(api::routes());
    rocket::build().mount("/api", routes)
}
