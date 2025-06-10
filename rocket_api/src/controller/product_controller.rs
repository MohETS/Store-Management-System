use crate::database::DbPool;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, routes};
use shared::Product;


#[get("/products")]
pub fn get_all_products(pool: &State<DbPool>) -> Json<Vec<Product>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let products = Product::get_all_products(&mut conn)
        .expect("Error loading products");

    Json(products)
}

#[get("/products/id/<id>")]
pub fn get_product_by_id(id: i32, pool: &State<DbPool>) -> Json<Product> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let product = Product::search_product_by_id(&mut conn, id)
        .expect("Error loading products");

    Json(product)
}

#[get("/products/name/<name>")]
pub fn get_product_by_name(name: &str, pool: &State<DbPool>) -> Json<Vec<Product>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let products = Product::search_product_by_name(&mut conn, name)
        .expect("Error loading products");

    Json(products)
}

#[get("/products/category/<category>")]
pub fn get_product_by_category(category: &str, pool: &State<DbPool>) -> Json<Vec<Product>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    let products = Product::search_product_by_category(&mut conn, category)
        .expect("Error loading products");

    Json(products)
}


pub fn routes() -> Vec<rocket::Route> {
    routes![get_all_products,get_product_by_id,get_product_by_name,get_product_by_category]
}
