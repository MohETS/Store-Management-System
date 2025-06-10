use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::{get, routes, State};
use rocket::http::Status;
use rocket::response::status::{Custom, Created};
use serde::{Deserialize};
use shared::Sale;

#[get("/sales")]
pub fn get_all_sales(pool: &State<DbPool>) -> Json<Vec<Sale>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let products = Sale::get_all_sales(&mut conn)
        .expect("Error loading products");

    Json(products)
}

#[post("/make-sale", data = "<sale_data>")]
pub fn make_sale_handler(pool: &State<DbPool>, sale_data: Json<MakeSaleRequest>) -> Result<Created<()>, Custom<String>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Sale::make_sale(&mut conn, sale_data.product_id, sale_data.quantity_sold) {
        Ok((_sale, _updated_product)) => {
            Ok(Created::new("/make_sale"))
        }
        Err(err) => {
            let msg = format!("Sale failed: {}", err);
            Err(Custom(Status::BadRequest, msg))
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_all_sales, make_sale_handler]
}


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MakeSaleRequest {
    pub product_id: i32,
    pub quantity_sold: i32,
}
