
use rocket::Route;

// src/routes/mod.rs
pub mod routes;
pub mod user;
pub mod admin_route;


pub fn create_routes() -> Vec<Route> {
    let mut all_routes = Vec::new();
    all_routes.extend(routes::create_routes());
    all_routes.extend(user::user_routes());
    all_routes.extend(admin_route::admin_routes());
    all_routes
}