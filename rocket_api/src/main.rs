#[macro_use]
extern crate rocket;
mod controller;
mod route;
mod database;
mod server;
mod cache_controller;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    server::run_server()
}






