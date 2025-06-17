use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, routes};
use rocket::http::Status;
use rocket::response::status::Custom;
use shared::SaleItem;
use shared::api_response::ApiResponse;

#[get("/sale-items")]
pub fn get_all_sale_items(pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<SaleItem>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match SaleItem::get_all_sale_items(&mut conn) {
        Ok(sale_items) => {
            let message = "Sale items Found".to_string();
            Custom(Status::Found, Json(ApiResponse::with_data(message, sale_items)))
        }
        Err(err) => {
            let message = format!("No sale items were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(message)))
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_all_sale_items]
}
