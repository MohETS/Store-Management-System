use rocket::{Rocket};
use crate::database;
use crate::route::all_routes;



pub fn run_server() -> Rocket<rocket::Build> {
    rocket::build().manage(database::init_database_connection_pool()).mount("/", all_routes())
}