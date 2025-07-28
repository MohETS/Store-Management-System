use std::sync::Once;
use rocket::local::blocking::Client;
use rocket::http::Status;

use rocket_api;
use shared::api_response::ApiResponse;
use shared::Product;

static INIT: Once = Once::new();

#[cfg(test)]
pub fn initialize() {
    INIT.call_once(|| {
        dotenv::from_filename(".env.test").ok();
    });
}

#[test]
fn test_get_all_products() {
    initialize();
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    let response = client.get("/products").dispatch();
    assert_eq!(response.status(), Status::Found);

    let body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<Product>> = serde_json::from_str(&body_str).expect("valid JSON response");
    let response_data: Vec<Product> = parsed_response.data.expect("Some vector of products");
    assert!(!response_data.is_empty());
}

#[test]
fn test_get_product_by_id() {
    initialize();
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    //Look for existing product
    let mut product_id = 1;
    let mut response = client.get(format!("/products/id/{product_id}")).dispatch();
    assert_eq!(response.status(), Status::Found);

    let mut body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Product> = serde_json::from_str(&body_str).expect("valid JSON response");
    let response_data: Product = parsed_response.data.expect("Some product");
    assert_eq!(response_data.name, "Sifu TEST");
    assert_eq!(response_data.category, "Games");

    //Look for non-existing product
    product_id = -1;
    response = client.get(format!("/products/id/{product_id}")).dispatch();
    assert_eq!(response.status(), Status::NotFound);

    body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Product> = serde_json::from_str(&body_str).expect("valid JSON response");
    assert!(parsed_response.message.contains("No product were found"));
}

#[test]
fn test_get_product_by_name() {
    initialize();
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    //Look for existing products
    let mut product_name = "Sifu";
    let mut response = client.get(format!("/products/name/{product_name}")).dispatch();
    assert_eq!(response.status(), Status::Found);

    let mut body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<Product>> = serde_json::from_str(&body_str).expect("valid JSON response");
    let response_data: Vec<Product> = parsed_response.data.expect("Some product");
    assert!(response_data[0].name.contains("Sifu"));
    assert!(!response_data.is_empty());

    //Look for non-existing products
    product_name = "Non-Existing";
    response = client.get(format!("/products/name/{product_name}")).dispatch();
    assert_eq!(response.status(), Status::NoContent);

    body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<Product>> = serde_json::from_str(&body_str).expect("valid JSON response");
    assert!(parsed_response.message.contains("No products Found"));
    assert!(parsed_response.data.is_none());
}


#[test]
fn test_get_product_by_category() {
    initialize();
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    //Look for existing products
    let mut product_category = "Sound";
    let mut response = client.get(format!("/products/category/{product_category}")).dispatch();
    assert_eq!(response.status(), Status::Found);

    let mut body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<Product>> = serde_json::from_str(&body_str).expect("valid JSON response");
    let response_data: Vec<Product> = parsed_response.data.expect("Some product");
    assert_eq!(response_data[0].category, "Sound");
    assert!(!response_data.is_empty());

    //Look for non-existing products
    product_category = "Non-Existing";
    response = client.get(format!("/products/category/{product_category}")).dispatch();
    assert_eq!(response.status(), Status::NoContent);

    body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<Product>> = serde_json::from_str(&body_str).expect("valid JSON response");
    assert!(parsed_response.message.contains("No products Found"));
    assert!(parsed_response.data.is_none());
}

#[test]
fn test_update_product() {
    initialize();
    let product_update: Product = Product {id: 1, category: String::from("Games"), name: String::from("Sifu TEST"), price: 50, quantity: 999};
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    //Look for existing products
    let mut response = client.post("/products/update").json(&product_update).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let mut body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<Product>> = serde_json::from_str(&body_str).expect("valid JSON response");
    assert_eq!(parsed_response.message, "Product updated");

    //Check if product was updated
    let product_id = 1;
    response = client.get(format!("/products/id/{product_id}")).dispatch();
    assert_eq!(response.status(), Status::Found);

    body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Product> = serde_json::from_str(&body_str).expect("valid JSON response");
    let response_data: Product = parsed_response.data.expect("Some product");
    assert_eq!(response_data.name, "Sifu TEST");
    assert_eq!(response_data.quantity, 999);
}


