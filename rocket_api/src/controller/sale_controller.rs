use crate::database::DbPool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, delete, State};
use rocket::response::status::Custom;
use rocket_okapi::openapi;
use shared::api_response::ApiResponse;
use shared::model::MakeSaleRequest;
use shared::Sale;

#[openapi(tag = "Sales")]
#[get("/sales")]
pub async fn get_all_sales(pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<Sale>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Sale::get_all_sales(&mut conn) {
        Ok(sales) => {
            let message = "Sales Found".to_string();
            Custom(Status::Found, Json(ApiResponse::with_data(Status::Found.to_string(), message, false, sales, String::from("/sales"))))
        }
        Err(err) => {
            let message = format!("No sales were found: {}", err);
            Custom(Status::Found, Json(ApiResponse::new(Status::NotFound.to_string(), message, String::from("/sales"))))
        }
    }
}

#[openapi(tag = "Sales")]
#[post("/make-sale", data = "<sale_data>")]
pub async fn make_sale_handler(pool: &State<DbPool>, sale_data: Json<MakeSaleRequest>) -> Custom<Json<ApiResponse<String>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Sale::make_sale(&mut conn, sale_data.product_id, sale_data.quantity_sold) {
        Ok((_sale, _updated_product)) => {
            let message = "Sale created".to_string();
            Custom(Status::Created, Json(ApiResponse::new(Status::Created.to_string(), message, String::from("/make-sale"))))
        }
        Err(err) => {
            let message = format!("Sale failed - {}", err);
            Custom(Status::BadRequest, Json(ApiResponse::new(Status::BadRequest.to_string(), message, String::from("/make-sale"))))
        }
    }
}

#[openapi(tag = "Sales")]
#[delete("/cancel-sale/id/<id>")]
pub async fn cancel_sale(id: i32, pool: &State<DbPool>) -> Custom<Json<ApiResponse<String>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Sale::cancel_sale(&mut conn, id, 0) {
        Ok(result) if result > 0 => {
            let message = "Sale removed".to_string();
            Custom(Status::Ok, Json(ApiResponse::new(Status::Ok.to_string(), message, format!("cancel-sale/id/{id}"))))
        }
        _ => {
            let message = format!("Sale failed - {}", "The selected sale doesn't exist");
            Custom(Status::BadRequest, Json(ApiResponse::new(Status::BadRequest.to_string(), message, format!("cancel-sale/id/{id}"))))
        }
    }
    // match Sale::delete_sale(&mut conn, id) {
    //     Ok(0) => {
    //         Ok(Created::new("/make_sale"))
    //     }
    //     Err(err) => {
    //         let message = format!("Sale failed: {}", err);
    //         Err(Custom(Status::BadRequest, message))
    //     }
    // }
}
