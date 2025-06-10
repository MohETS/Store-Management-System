use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, routes};

use shared::SaleItem;

#[get("/sale-items")]
pub fn get_all_sale_items(pool: &State<DbPool>) -> Json<Vec<SaleItem>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let products = SaleItem::get_all_sale_items(&mut conn)
        .expect("Error loading products");

    Json(products)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_all_sale_items]
}
