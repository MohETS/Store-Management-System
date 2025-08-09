use rocket::http::Status;
use rocket::local::blocking::Client;
use std::sync::Once;
use rocket::form::validate::Contains;
use rocket_api;
use shared::api_response::ApiResponse;
use shared::model::MakeSaleRequest;
use shared::Sale;

static INIT: Once = Once::new();

#[cfg(test)]
pub fn initialize() {
    INIT.call_once(|| {
        dotenv::from_filename(".env.test").ok();
    });
}

#[test]
fn test_get_all_sales() {
    initialize();
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    let response = client.get("/sales").dispatch();
    assert_eq!(response.status(), Status::Found);

    let body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<Sale>> = serde_json::from_str(&body_str).expect("valid JSON response");
    let response_data: Vec<Sale> = parsed_response.data.expect("Some vector of sales");
    assert!(!response_data.is_empty());
}


#[test]
fn test_make_sale() {
    initialize();
    let new_sale: MakeSaleRequest = MakeSaleRequest { product_id: 1, quantity_sold: 1 };
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    let response = client.post("/make-sale").json(&new_sale).dispatch();
    assert_eq!(response.status(), Status::Created);

    let body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<Vec<Sale>> = serde_json::from_str(&body_str).expect("valid JSON response");
    assert_eq!(parsed_response.message, "Sale created");
}

#[test]
fn test_cancel_sale() {
    initialize();
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    let mut response = client.get("/sales").dispatch();

    let mut body_str = response.into_string().expect("response body");
    let parsed_response: ApiResponse<Vec<Sale>> = serde_json::from_str(&body_str).expect("valid JSON response");
    let response_data: Vec<Sale> = parsed_response.data.expect("Some vector of sales");
    let sale_id = response_data[response_data.len() - 1].id;

    // Successful sale deletion
    response = client.delete(format!("/cancel-sale/id/{sale_id}")).dispatch();
    assert_eq!(response.status(), Status::Ok);

    body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<String> = serde_json::from_str(&body_str).expect("valid JSON response");
    assert_eq!(parsed_response.message, "Sale removed");


    // Failed sale deletion
    response = client.delete("/cancel-sale/id/-1").dispatch();
    assert_eq!(response.status(), Status::BadRequest);

    body_str = response.into_string().expect("response body");

    assert!(body_str.contains("Sale failed"));
}