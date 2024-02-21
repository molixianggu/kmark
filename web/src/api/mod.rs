mod users;

pub fn routes() -> Vec<rocket::Route> {
    let mut routes = vec![];
    routes.extend(users::users_routes());
    routes
}
