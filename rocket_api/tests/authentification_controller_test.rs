use rocket::http::Status;
use rocket::local::blocking::Client;
use std::sync::Once;
use rocket_api;
use shared::api_response::ApiResponse;
use shared::StoreAccount;

static INIT: Once = Once::new();

#[cfg(test)]
pub fn initialize() {
    INIT.call_once(|| {
        dotenv::from_filename(".env.test").ok();
    });
}

#[test]
fn test_store_account_login_successful() {
    initialize();
    let test_account: StoreAccount = StoreAccount { username: String::from("test.store"), password: String::from("password") };
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    let response = client.post("/login").json(&test_account).dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<String> = serde_json::from_str(&body_str).expect("valid JSON response");
    assert_eq!(parsed_response.message, "Login Successful");
}

#[test]
fn test_store_account_login_unsuccessful() {
    initialize();
    let test_account: StoreAccount = StoreAccount { username: String::from("fake account"), password: String::from("test") };
    let client = Client::tracked(rocket_api::run_server()).expect("valid rocket instance");

    let response = client.post("/login").json(&test_account).dispatch();
    assert_eq!(response.status(), Status::Unauthorized);

    let body_str = response.into_string().expect("response body");

    let parsed_response: ApiResponse<String> = serde_json::from_str(&body_str).expect("valid JSON response");
    assert_eq!(parsed_response.message, "Login Failure");
}
