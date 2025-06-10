use rocket::Route;
use crate::controller::*;

pub fn all_routes() -> Vec<Route> {
    let mut routes:Vec<Route> = Vec::new();

    routes.extend(product_controller::routes());
    routes.extend(sale_controller::routes());
    routes.extend(sale_item_controller::routes());


    routes
}