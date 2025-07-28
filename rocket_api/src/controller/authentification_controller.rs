use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{post};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_okapi::openapi;
use shared::StoreAccount;
use shared::api_response::ApiResponse;

#[openapi(tag = "Login")]
#[post("/login", data = "<login_credentials>")]
pub fn store_login_authentification(pool: &State<DbPool>, login_credentials: Json<StoreAccount>) -> Custom<Json<ApiResponse<String>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match StoreAccount::login_authentification(&mut conn, &login_credentials.username, &login_credentials.password) {
        true => {
            let message = "Login Successful".to_string();
            Custom(Status::Ok, Json(ApiResponse::new(Status::Ok.to_string(), message, String::from("/login"))))
        }
        false => {
            let message = "Login Failure".to_string();
            Custom(Status::Unauthorized, Json(ApiResponse::new(Status::Unauthorized.to_string(), message, String::from("/login"))))
        }
    }
}