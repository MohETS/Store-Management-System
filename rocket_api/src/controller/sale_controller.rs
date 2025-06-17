use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::{get, routes, State};
use rocket::http::Status;
use rocket::response::status::{Custom};
use shared::model::MakeSaleRequest;
use shared::Sale;
use shared::api_response::ApiResponse;

#[get("/sales")]
pub fn get_all_sales(pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<Sale>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Sale::get_all_sales(&mut conn) {
        Ok(sales) => {
            let message = "Sales Found".to_string();
            Custom(Status::Found, Json(ApiResponse::with_data(message, sales)))
        }
        Err(err) => {
            let message = format!("No sales were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(message)))
        }
    }
}

#[post("/make-sale", data = "<sale_data>")]
pub fn make_sale_handler(pool: &State<DbPool>, sale_data: Json<MakeSaleRequest>) -> Custom<String> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Sale::make_sale(&mut conn, sale_data.product_id, sale_data.quantity_sold) {
        Ok((_sale, _updated_product)) => {
            let msg = "Sale created".to_string();
            Custom(Status::Created, msg)
        }
        Err(err) => {
            let msg = format!("Sale failed - {}", err);
            Custom(Status::BadRequest, msg)
        }
    }
}

#[delete("/cancel-sale/id/<id>")]
pub fn cancel_sale(id: i32, pool: &State<DbPool>) -> Custom<String> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Sale::cancel_sale(&mut conn, id, 0) {
        Ok(result) if result > 0 => {
            let msg = "Sale removed".to_string();
            Custom(Status::Gone, msg)
        }
        _ => {
            let msg = format!("Sale failed - {}", "The selected sale doesn't exist");
            Custom(Status::BadRequest, msg)
        }
    }
    // match Sale::delete_sale(&mut conn, id) {
    //     Ok(0) => {
    //         Ok(Created::new("/make_sale"))
    //     }
    //     Err(err) => {
    //         let msg = format!("Sale failed: {}", err);
    //         Err(Custom(Status::BadRequest, msg))
    //     }
    // }
}


pub fn routes() -> Vec<rocket::Route> {
    routes![get_all_sales, make_sale_handler, cancel_sale]
}
