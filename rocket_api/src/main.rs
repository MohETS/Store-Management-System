#[macro_use]
extern crate rocket;
mod controller;
mod route;
mod database;
mod server;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    server::run_server()
}






