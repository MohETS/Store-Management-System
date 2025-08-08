use moka::future::Cache;
use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_okapi::openapi;
use shared::Product;
use shared::api_response::ApiResponse;

#[openapi(tag = "Products")]
#[get("/products")]
pub async fn get_all_products(pool: &State<DbPool>, cache: &State<Cache<String, Vec<Product>>>) -> Custom<Json<ApiResponse<Vec<Product>>>> {
    if let Some(cached_products) = cache.get("all_products").await {
        let message = "Products Found".to_string();
        return Custom(Status::Found, Json(ApiResponse::with_data(Status::Found.to_string(), message, true, cached_products, String::from("/products"))))
    }


    let mut conn = pool.get().expect("Failed to get DB connection");
    match Product::get_all_products(&mut conn) {
        Ok(product) => {
            cache.insert("all_products".to_string(), product.clone()).await;
            let message = "Products Found".to_string();
            Custom(Status::Found, Json(ApiResponse::with_data(Status::Found.to_string(), message, false, product, String::from("/products"))))
        }
        Err(err) => {
            let message = format!("No products were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(Status::NotFound.to_string(), message, String::from("/products"))))
        }
    }
}

#[openapi(tag = "Products")]
#[get("/products/id/<id>")]
pub async fn get_product_by_id(id: i32, pool: &State<DbPool>) -> Custom<Json<ApiResponse<Product>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Product::search_product_by_id(&mut conn, id) {
        Ok(product) => {
            let message = "Product Found".to_string();
            Custom(Status::Found, Json(ApiResponse::with_data(Status::Found.to_string(), message, false, product, format!("/products/id/{id}"))))
        }
        Err(err) => {
            let message = format!("No product were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(Status::NotFound.to_string(), message, format!("/products/id/{id}"))))
        }
    }
}

#[openapi(tag = "Products")]
#[get("/products/name/<name>")]
pub async fn get_product_by_name(name: &str, pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<Product>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Product::search_product_by_name(&mut conn, name) {
        Ok(product) => {
            if product.is_empty() {
                let message = "No products Found".to_string();
                Custom(Status::NoContent, Json(ApiResponse::new(Status::NoContent.to_string(), message, format!("/products/name/{name}"))))
            }else{
                let message = "Products Found".to_string();
                Custom(Status::Found, Json(ApiResponse::with_data(Status::Found.to_string(), message, false, product, format!("/products/name/{name}"))))
            }
        }
        Err(err) => {
            let message = format!("Error: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(Status::NotFound.to_string(), message, format!("/products/name/{name}"))))
        }
    }
}

#[openapi(tag = "Products")]
#[get("/products/category/<category>")]
pub async fn get_product_by_category(category: &str, pool: &State<DbPool>) -> Custom<Json<ApiResponse<Vec<Product>>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Product::search_product_by_category(&mut conn, category) {
        Ok(product) => {
            if product.is_empty() {
                let message = "No products Found".to_string();
                Custom(Status::NoContent, Json(ApiResponse::new(Status::NoContent.to_string(), message, format!("/products/category/{category}"))))
            }else{
                let message = "Products Found".to_string();
                Custom(Status::Found, Json(ApiResponse::with_data(Status::Found.to_string(), message, false, product, format!("/products/category/{category}"))))
            }
        }
        Err(err) => {
            let message = format!("No product were found: {}", err);
            Custom(Status::NotFound, Json(ApiResponse::new(Status::NotFound.to_string(), message, format!("/products/category/{category}"))))
        }
    }
}

#[openapi(tag = "Products")]
#[post("/products/update", data = "<product_data>")]
pub async fn update_product(pool: &State<DbPool>, cache: &State<Cache<String, Vec<Product>>>, product_data: Json<Product>) -> Custom<Json<ApiResponse<String>>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    match Product::update_product(&mut conn, product_data.into_inner()) {
        Ok(result) if result > 0 => {
            cache.invalidate("all_products").await;
            let message = "Product updated".to_string();
            Custom(Status::Ok, Json(ApiResponse::new(Status::Ok.to_string(), message, String::from("/products/update"))))
        }
        _ => {
            let message = format!("Product did not update - {}", "The selected sale doesn't exist");
            Custom(Status::BadRequest, Json(ApiResponse::new(Status::BadRequest.to_string(), message, String::from("/products/update"))))
        }
    }
}
