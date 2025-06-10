mod controller;
mod route;
mod database;

#[macro_use]
extern crate rocket;
use route::all_routes;

#[launch]
fn rocket() -> _ {
    rocket::build().manage(database::init_database_connection_pool()).mount("/", all_routes())
}









