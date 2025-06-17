use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, routes};
use rocket::http::Status;
use rocket::response::status::Custom;
use shared::Product;
use shared::api_response::ApiResponse;

#[get("/products")]
pub fn get_all_products(pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<Product>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Product::get_all_products(&mut conn) {
        Ok(product) => {
            let message = "Products Found".to_string();
            Custom(Status::Found, Json(ApiResponse::with_data(message, product)))
        }
        Err(err) => {
            let message = format!("No products were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(message)))
        }
    }
}

#[get("/products/id/<id>")]
pub fn get_product_by_id(id: i32, pool: &State<DbPool>) -> Custom<Json<ApiResponse<Product>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Product::search_product_by_id(&mut conn, id) {
        Ok(product) => {
            let message = "Product Found".to_string();
            Custom(Status::Found, Json(ApiResponse::with_data(message, product)))
        }
        Err(err) => {
            let message = format!("No product were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(message)))
        }
    }
}

#[get("/products/name/<name>")]
pub fn get_product_by_name(name: &str, pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<Product>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Product::search_product_by_name(&mut conn, name) {
        Ok(product) => {
            if product.is_empty() {
                let message = "No products Found".to_string();
                Custom(Status::Found, Json(ApiResponse::new(message)))
            }else{
                let message = "Products Found".to_string();
                Custom(Status::Found, Json(ApiResponse::with_data(message, product)))
            }
        }
        Err(err) => {
            let message = format!("Error: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(message)))
        }
    }
}

#[get("/products/category/<category>")]
pub fn get_product_by_category(category: &str, pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<Product>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Product::search_product_by_category(&mut conn, category) {
        Ok(product) => {
            if product.is_empty() {
                let message = "No products Found".to_string();
                Custom(Status::Found, Json(ApiResponse::new(message)))
            }else{
                let message = "Products Found".to_string();
                Custom(Status::Found, Json(ApiResponse::with_data(message, product)))
            }
        }
        Err(err) => {
            let message = format!("No product were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(message)))
        }
    }
}


pub fn routes() -> Vec<rocket::Route> {
    routes![get_all_products, get_product_by_id, get_product_by_name, get_product_by_category]
}
