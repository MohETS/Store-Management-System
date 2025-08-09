use crate::controller::*;
use rocket::Route;
use rocket_okapi::openapi_get_routes;

pub fn all_routes() -> Vec<Route> {
    let mut routes: Vec<Route> = Vec::new();

    // routes.extend(product_controller::routes());
    // routes.extend(sale_controller::routes());
    // routes.extend(sale_item_controller::routes());

    routes.extend(openapi_get_routes![
        //Authentification Routes
        authentification_controller::store_login_authentification,
        //Product Routes
        product_controller::get_all_products,
        product_controller::get_product_by_id,
        product_controller::get_product_by_name,
        product_controller::get_product_by_category,
        product_controller::update_product,
        //Sale Routes
        sale_controller::get_all_sales,
        sale_controller::make_sale_handler,
        sale_controller::cancel_sale,
        //Sale Items Routes
        sale_item_controller::get_all_sale_items
    ]);
    routes
}