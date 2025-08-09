use rocket::http::Status;
use rocket::local::blocking::Client;
use std::sync::Once;
use rocket_api;
use shared::api_response::ApiResponse;
use shared::{SaleItem};

static INIT: Once = Once::new();

#[cfg(test)]
pub fn initialize() {
    INIT.call_once(|| {
        dotenv::from_filename(".env.test").ok();
    });
}

#[test]
fn test_get_all_sale_items() {
    initialize();
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    let response = client.get("/sale-items").dispatch();
    assert_eq!(response.status(), Status::Found);

    let body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<SaleItem>> = serde_json::from_str(&body_str).expect("valid JSON response");
    let response_data: Vec<SaleItem> = parsed_response.data.expect("Some vector of sale items");
    assert!(!response_data.is_empty());
}