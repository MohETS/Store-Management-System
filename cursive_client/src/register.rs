use crate::{Product, Sale, SaleItem};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, ListView, SelectView, TextView};
use cursive::Cursive;
use diesel::PgConnection;
use std::sync::{Arc, Mutex};


pub struct Register {
    conn: Arc<Mutex<PgConnection>>,
}


impl Register {
    pub fn new(conn: PgConnection) -> Self {
        Register {
            conn: Arc::new(Mutex::new(conn))
        }
    }

    pub fn setup_ui(&mut self, siv: &mut Cursive) {
        siv.set_user_data(self.conn.clone());
        siv.add_layer(
            Dialog::around(
                SelectView::new()
                    .item("Search Products", 1)
                    .item("Make Sale", 2)
                    .item("Cancel Sale", 3)
                    .item("View All Products", 4)
                    .item("View All Sales", 5)
                    .item("View All Sale Items", 6)
                    .on_submit(|s, item| {
                        match *item {
                            1 => Register::show_search_menu(s),
                            2 => Register::show_make_sale(s),
                            3 => Register::show_cancel_sale(s),
                            4 => Register::show_all_products(s),
                            5 => Register::show_all_sales(s),
                            6 => Register::show_all_sale_items(s),
                            _ => {}
                        }
                    })
            ).title("LOG430 - Store Management").button("Quit", |s| s.quit())
        );
    }

    /* UC-01 */
    fn show_search_menu(siv: &mut Cursive) {
        siv.add_layer(
            Dialog::around(
                SelectView::new()
                    .item("Search by ID", 1)
                    .item("Search by Name", 2)
                    .item("Search by Category", 3)
                    .on_submit(|s, item| {
                        match *item {
                            1 => Register::search_by_id(s),
                            2 => Register::search_by_name(s),
                            3 => Register::search_by_category(s),
                            _ => {}
                        }
                    })
            ).title("LOG430 - Search Products").button("Back", |s| { s.pop_layer(); })
        );
    }

    fn search_by_id(siv: &mut Cursive) {
        siv.add_layer(
            Dialog::around(
                EditView::new().with_name("id_input")
            ).title("LOG430 - Enter Product ID")
                .button("Search", |s| {
                    let id = s.call_on_name("id_input", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                    if let Ok(parsed_id) = id.parse::<i32>() {
                        let conn = s.user_data::<Arc<Mutex<PgConnection>>>().unwrap().clone();
                        let mut conn = conn.lock().unwrap();
                        let product = Product::search_product_by_id(&mut conn, parsed_id);
                        match product {
                            Ok(product) => {
                                let result: Vec<Product> = vec![product];
                                Register::display_search_results(s, result);
                            }
                            Err(_) => {
                                s.add_layer(Dialog::info("Product not found"));
                            }
                        }
                    }
                }).button("Back", |s| { s.pop_layer(); })
        );
    }

    fn search_by_name(siv: &mut Cursive) {
        siv.add_layer(
            Dialog::around(
                EditView::new().with_name("name_input")
            )
                .title("LOG430 - Enter Product Name")
                .button("Search", |s| {
                    let name = s.call_on_name("name_input", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                    let conn = s.user_data::<Arc<Mutex<PgConnection>>>().unwrap().clone();
                    let mut conn = conn.lock().unwrap();
                    let products = Product::search_product_by_name(&mut conn, &name);
                    match products {
                        Ok(products) => {
                            Register::display_search_results(s, products);
                        }
                        Err(_) => {
                            s.add_layer(Dialog::info("Product not found"));
                        }
                    }
                })
                .button("Back", |s| { s.pop_layer(); })
        );
    }

    fn search_by_category(siv: &mut Cursive) {
        siv.add_layer(
            Dialog::around(
                EditView::new().with_name("category_input")
            )
                .title("LOG430 - Enter Product Category")
                .button("Search", |s| {
                    let name = s.call_on_name("category_input", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                    let conn = s.user_data::<Arc<Mutex<PgConnection>>>().unwrap().clone();
                    let mut conn = conn.lock().unwrap();
                    let products = Product::search_product_by_category(&mut conn, &name);
                    match products {
                        Ok(products) => {
                            Register::display_search_results(s, products);
                        }
                        Err(_) => {
                            s.add_layer(Dialog::info("Product not found"));
                        }
                    }
                })
                .button("Back", |s| { s.pop_layer(); })
        );
    }

    fn display_search_results(siv: &mut Cursive, products: Vec<Product>) {
        let mut list = ListView::new();

        for product in products {
            list.add_child(format!("(ID:{})", product.id),
                           TextView::new(format!("Name: {}, Category: {}, Quantity: {}, Price: {}$",
                                                 product.name, product.category, product.quantity, product.price)));
        }

        siv.add_layer(
            Dialog::around(list)
                .title("LOG430 - Search Results")
                .button("Back", |s| { s.pop_layer(); })
        );
    }


    /* UC-02 */
    fn show_make_sale(siv: &mut Cursive) {
        siv.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new("Product ID:"))
                    .child(EditView::new().with_name("sale_product_id"))
                    .child(TextView::new("Quantity:"))
                    .child(EditView::new().with_name("sale_quantity"))
            )
                .title("Make Sale")
                .button("Confirm Sale", |s| {
                    let product_id = s.call_on_name("sale_product_id", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                    let quantity = s.call_on_name("sale_quantity", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();

                    Register::process_sale(s, product_id.parse().unwrap_or(0), quantity.parse().unwrap_or(0));
                })
                .button("Back", |s| { s.pop_layer(); })
        );
    }

    fn process_sale(siv: &mut Cursive, product_id: i32, quantity: i32) {
        let conn = siv.user_data::<Arc<Mutex<PgConnection>>>().unwrap().clone();
        let mut conn = conn.lock().unwrap();

        match Sale::make_sale(&mut conn, product_id, quantity) {
            Ok(_) => {
                siv.add_layer(
                    Dialog::info("Sale completed successfully!")
                );
            }
            Err(_) => {
                siv.add_layer(
                    Dialog::info("Sale failed")
                );
            }
        }
    }


    /* UC-03 */
    fn show_cancel_sale(siv: &mut Cursive) {
        siv.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new("Sale ID:"))
                    .child(EditView::new().with_name("sale_id"))
                // .child(TextView::new("Product ID:"))
                // .child(EditView::new().with_name("product_id"))
            )
                .title("LOG430 - Cancel Sale")
                .button("Cancel Sale", |s| {
                    let sale_id = s.call_on_name("sale_id", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                    // let product_id = s.call_on_name("product_id", |view: &mut EditView| {
                    //     view.get_content()
                    // }).unwrap();

                    Register::process_cancel_sale(s, sale_id.parse().unwrap_or(0), 0);
                })
                .button("Back", |s| { s.pop_layer(); })
        );
    }

    fn process_cancel_sale(siv: &mut Cursive, sale_id: i32, product_id: i32) {
        let conn = siv.user_data::<Arc<Mutex<PgConnection>>>().unwrap().clone();
        let mut conn = conn.lock().unwrap();

        match Sale::cancel_sale(&mut conn, sale_id, product_id) {
            Ok(result) if result > 0 => {
                siv.add_layer(
                    Dialog::info("Sale cancelled successfully!")
                );
            }
            _ => {
                siv.add_layer(
                    Dialog::info("No sale was cancelled")
                );
            }
        }
    }


    /* UC-04 */
    fn show_all_products(siv: &mut Cursive) {
        let mut list = ListView::new();

        let conn = siv.user_data::<Arc<Mutex<PgConnection>>>().unwrap().clone();
        let mut conn = conn.lock().unwrap();
        let products = Product::get_all_products(&mut conn).unwrap();

        for product in products {
            list.add_child(format!("ID: {}", product.id),
                           TextView::new(format!("Name: {}, Category: {}, Quantity: {}, Price: {}$",
                                                 product.name, product.category, product.quantity, product.price)));
        }

        siv.add_layer(
            Dialog::around(list)
                .title("LOG430 - All Products")
                .button("Back", |s| { s.pop_layer(); })
        );
    }


    /* General function */
    fn show_all_sales(siv: &mut Cursive) {
        let mut list = ListView::new();

        let conn = siv.user_data::<Arc<Mutex<PgConnection>>>().unwrap().clone();
        let mut conn = conn.lock().unwrap();
        let sales = Sale::get_all_sales(&mut conn).unwrap();

        for sale in sales {
            list.add_child(format!("ID: {}", sale.id),
                           TextView::new(format!("Total Price: {}$", sale.total_price)));
        }

        siv.add_layer(
            Dialog::around(list)
                .title("LOG430 - All Sales")
                .button("Back", |s| { s.pop_layer(); })
        );
    }

    fn show_all_sale_items(siv: &mut Cursive) {
        let mut list = ListView::new();

        let conn = siv.user_data::<Arc<Mutex<PgConnection>>>().unwrap().clone();
        let mut conn = conn.lock().unwrap();
        let sale_items = SaleItem::get_all_sale_items(&mut conn).unwrap();

        for item in sale_items {
            list.add_child(format!("ID: {}", item.id),
                           TextView::new(format!("Sale ID: {} Product ID: {} Quantity: {} Product Price: {}$"
                                                 , item.sale_id, item.product_id, item.quantity, item.product_price)));
        }

        siv.add_layer(
            Dialog::around(list)
                .title("LOG430 - All Sales")
                .button("Back", |s| { s.pop_layer(); })
        );
    }
}
