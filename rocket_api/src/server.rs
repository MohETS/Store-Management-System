use crate::database;
use crate::route::all_routes;
use rocket::Rocket;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

pub fn run_server() -> Rocket<rocket::Build> {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://127.0.0.1:8000",
        "http://localhost:8000",
        "http://127.0.0.1",
        "http://localhost"
    ]);
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: ["GET", "POST", "PUT", "DELETE"]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("Failed to create CORS fairing");

    rocket::build().attach(cors).manage(database::init_database_connection_pool()).mount("/", all_routes())
        .mount("/doc/", make_swagger_ui(&SwaggerUIConfig { url: "../openapi.json".to_owned(), ..Default::default() }))
}