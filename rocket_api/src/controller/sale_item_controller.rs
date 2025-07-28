use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_okapi::openapi;
use shared::SaleItem;
use shared::api_response::ApiResponse;

#[openapi(tag = "Sale Items")]
#[get("/sale-items")]
pub fn get_all_sale_items(pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<SaleItem>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match SaleItem::get_all_sale_items(&mut conn) {
        Ok(sale_items) => {
            let message = "Sale items Found".to_string();
            Custom(Status::Found, Json(ApiResponse::with_data(Status::Found.to_string(), message, sale_items, String::from("/sale-items"))))
        }
        Err(err) => {
            let message = format!("No sale items were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(Status::NotFound.to_string(), message, String::from("/sale-items"))))
        }
    }
}
