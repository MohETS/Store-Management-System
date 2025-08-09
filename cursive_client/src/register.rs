use crate::{Product, Sale, SaleItem};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, ListView, SelectView, TextView};
use cursive::Cursive;
use cursive_async_view::AsyncView;
use reqwest::blocking::Client;
use shared::api_response::ApiResponse;
use shared::model::MakeSaleRequest;

pub struct Register {
    client: Client,
}

impl Register {
    pub fn new() -> Self {
        Register {
            client: Client::new(),
        }
    }

    pub fn setup_ui(&mut self, siv: &mut Cursive) {
        siv.set_user_data(self.client.clone());

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
                        process_search_by_id(s, parsed_id);
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
                    process_search_by_name(s, name.to_string());
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
                    let category = s.call_on_name("category_input", |view: &mut EditView| {
                        view.get_content()
                    }).unwrap();
                    process_search_by_category(s, category.to_string());
                })
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

                    process_sale(s, product_id.parse().unwrap_or(0), quantity.parse().unwrap_or(0));
                })
                .button("Back", |s| { s.pop_layer(); })
        );
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


                    process_cancel_sale(s, sale_id.parse().unwrap_or(0), 0);
                })
                .button("Back", |s| { s.pop_layer(); })
        );
    }


    /* UC-04 */
    fn show_all_products(siv: &mut Cursive) {
        let client: Client = {
            let client_ref = siv.user_data::<Client>().unwrap();
            client_ref.clone()
        };

        let async_view = AsyncView::new_with_bg_creator(
            siv,
            move || {
                let mut list = ListView::new();
                let response = client.get("http://localhost/products").send();

                match response {
                    Ok(response) => {
                        match response.json::<ApiResponse<Vec<Product>>>() {
                            Ok(api_response) => {
                                if let Some(products) = api_response.data {
                                    for product in products {
                                        list.add_child(format!("ID: {}", product.id),
                                                       TextView::new(format!("Name: {}, Category: {}, Quantity: {}, Price: {}$",
                                                                             product.name, product.category, product.quantity, product.price)));
                                    }
                                } else {
                                    list.add_child("", TextView::new(api_response.message));
                                }
                            }
                            Err(err) => {
                                list.add_child("Parse Error: ", TextView::new(format!("Failed to parse JSON - {}", err)));
                            }
                        }
                    }
                    Err(err) => {
                        list.add_child("Request Error: ", TextView::new(format!("{:?}", err)));
                    }
                }
                Ok(list)
            },
            move |list: ListView| {
                Dialog::around(list)
                    .title("LOG430 - All Products")
                    .button("Back", |s| {
                        s.pop_layer();
                    })
            },
        ).max_width(150);

        siv.add_layer(async_view);
    }


    /* General function */
    fn show_all_sales(siv: &mut Cursive) {
        let client: Client = {
            let client_ref = siv.user_data::<Client>().unwrap();
            client_ref.clone()
        };

        let async_view = AsyncView::new_with_bg_creator(
            siv,
            move || {
                let mut list = ListView::new();
                let response = client.get("http://localhost/sales").send();

                match response {
                    Ok(response) => {
                        match response.json::<ApiResponse<Vec<Sale>>>() {
                            Ok(api_response) => {
                                if let Some(sales) = api_response.data {
                                    for sale in sales {
                                        list.add_child(format!("ID: {}", sale.id),
                                                       TextView::new(format!("Total Price: {}$", sale.total_price)));
                                    }
                                } else {
                                    list.add_child("", TextView::new(api_response.message));
                                }
                            }
                            Err(err) => {
                                list.add_child("", TextView::new(format!("Failed to parse JSON - {}", err)));
                            }
                        }
                    }
                    Err(err) => {
                        list.add_child("Request Error: ", TextView::new(format!("{:?}", err)));
                    }
                }

                Ok(list)
            },
            move |list: ListView| {
                Dialog::around(list)
                    .title("LOG430 - All Sales")
                    .button("Back", |s| {
                        s.pop_layer();
                    })
            },
        ).max_width(150);
        siv.add_layer(async_view);
    }

    fn show_all_sale_items(siv: &mut Cursive) {
        let client: Client = {
            let client_ref = siv.user_data::<Client>().unwrap();
            client_ref.clone()
        };

        let async_view = AsyncView::new_with_bg_creator(
            siv,
            move || {
                let mut list = ListView::new();
                let response = client.get("http://localhost/sale-items").send();

                match response {
                    Ok(response) => {
                        match response.json::<ApiResponse<Vec<SaleItem>>>() {
                            Ok(api_response) => {
                                if let Some(sale_items) = api_response.data {
                                    for item in sale_items {
                                        list.add_child(format!("ID: {}", item.id),
                                                       TextView::new(format!("Sale ID: {} Product ID: {} Quantity: {} Product Price: {}$"
                                                                             , item.sale_id, item.product_id, item.quantity, item.product_price)));
                                    }
                                } else {
                                    list.add_child("", TextView::new(api_response.message));
                                }
                            }
                            Err(err) => {
                                list.add_child("Parse Error: ", TextView::new(format!("Failed to parse JSON - {}", err)));
                            }
                        }
                    }
                    Err(err) => {
                        list.add_child("Request Error: ", TextView::new(format!("{:?}", err)));
                    }
                }

                Ok(list)
            },
            move |list: ListView| {
                Dialog::around(list)
                    .title("LOG430 - All Sale Items")
                    .button("Back", |s| {
                        s.pop_layer();
                    })
            },
        ).max_width(150);

        siv.add_layer(async_view);
    }
}

fn process_search_by_id(siv: &mut Cursive, product_id: i32) {
    let client: Client = {
        let client_ref = siv.user_data::<Client>().unwrap();
        client_ref.clone()
    };

    let async_view = AsyncView::new_with_bg_creator(
        siv,
        move || {
            let mut list = ListView::new();
            let response = client.get(format!("http://localhost/products/id/{}", product_id)).send();

            match response {
                Ok(response) => {
                    match response.json::<ApiResponse<Product>>() {
                        Ok(api_response) => {
                            if let Some(product) = api_response.data {
                                list.add_child(format!("ID: {}", product.id),
                                               TextView::new(format!("Name: {}, Category: {}, Quantity: {}, Price: {}$",
                                                                     product.name, product.category, product.quantity, product.price)));
                            } else {
                                list.add_child("", TextView::new(api_response.message));
                            }
                        }
                        Err(err) => {
                            list.add_child("Parse Error: ", TextView::new(format!("Failed to parse JSON - {}", err)));
                        }
                    }
                }
                Err(err) => {
                    list.add_child("Request Error: ", TextView::new(format!("{:?}", err)));
                }
            }

            Ok(list)
        },
        move |list: ListView| {
            Dialog::around(list)
                .title("LOG430 - All Products")
                .button("Back", |s| {
                    s.pop_layer();
                })
        },
    ).max_width(150);

    siv.add_layer(async_view);
}


fn process_search_by_name(siv: &mut Cursive, product_name: String) {
    let client: Client = {
        let client_ref = siv.user_data::<Client>().unwrap();
        client_ref.clone()
    };

    let async_view = AsyncView::new_with_bg_creator(
        siv,
        move || {
            let mut list = ListView::new();
            let response = client.get(format!("http://localhost/products/name/{}", product_name)).send();

            match response {
                Ok(response) => {
                    match response.json::<ApiResponse<Vec<Product>>>() {
                        Ok(api_response) => {
                            if let Some(products) = api_response.data {
                                for product in products {
                                    list.add_child(format!("(ID:{})", product.id),
                                                   TextView::new(format!("Name: {}, Category: {}, Quantity: {}, Price: {}$",
                                                                         product.name, product.category, product.quantity, product.price)));
                                }
                            } else {
                                list.add_child("", TextView::new(api_response.message));
                            }
                        }
                        Err(err) => {
                            list.add_child("Parse Error: ", TextView::new(format!("Failed to parse JSON - {}", err)));
                        }
                    }
                }
                Err(err) => {
                    list.add_child("Request Error: ", TextView::new(format!("{:?}", err)));
                }
            }

            Ok(list)
        },
        move |list: ListView| {
            Dialog::around(list)
                .title("LOG430 - All Products")
                .button("Back", |s| {
                    s.pop_layer();
                })
        },
    ).max_width(150);

    siv.add_layer(async_view);
}


fn process_search_by_category(siv: &mut Cursive, product_category: String) {
    let client: Client = {
        let client_ref = siv.user_data::<Client>().unwrap();
        client_ref.clone()
    };

    let async_view = AsyncView::new_with_bg_creator(
        siv,
        move || {
            let mut list = ListView::new();
            let response = client.get(format!("http://localhost/products/category/{}", product_category)).send();

            match response {
                Ok(response) => {
                    match response.json::<ApiResponse<Vec<Product>>>() {
                        Ok(api_response) => {
                            if let Some(products) = api_response.data {
                                for product in products {
                                    list.add_child(format!("(ID:{})", product.id),
                                                   TextView::new(format!("Name: {}, Category: {}, Quantity: {}, Price: {}$",
                                                                         product.name, product.category, product.quantity, product.price)));
                                }
                            } else {
                                list.add_child("", TextView::new(api_response.message));
                            }
                        }
                        Err(err) => {
                            list.add_child("Parse Error: ", TextView::new(format!("Failed to parse JSON - {}", err)));
                        }
                    }
                }
                Err(err) => {
                    list.add_child("Request Error: ", TextView::new(format!("{:?}", err)));
                }
            }

            Ok(list)
        },
        move |list: ListView| {
            Dialog::around(list)
                .title("LOG430 - All Products")
                .button("Back", |s| {
                    s.pop_layer();
                })
        },
    ).max_width(150);

    siv.add_layer(async_view);
}


/* UC-02 */
fn process_sale(siv: &mut Cursive, product_id: i32, quantity_sold: i32) {
    let new_sale: MakeSaleRequest = MakeSaleRequest { product_id, quantity_sold };

    let client: Client = {
        let client_ref = siv.user_data::<Client>().unwrap();
        client_ref.clone()
    };

    let async_view = AsyncView::new_with_bg_creator(
        siv,
        move || {
            let response = client.post("http://localhost/make-sale").json(&new_sale).send();

            let message = match response {
                Ok(response) => {
                    let status = response.status();
                    if status == reqwest::StatusCode::CREATED {
                        response.text().unwrap()
                    } else {
                        format!("Error {}: {}", status, response.text().unwrap())
                    }
                }
                Err(err) => {
                    format!("{:?}", err)
                }
            };

            Ok(message)
        },
        move |message: String| {
            Dialog::info(message)
        },
    ).max_width(150);

    siv.add_layer(async_view);
}


/* UC-03 */
fn process_cancel_sale(siv: &mut Cursive, sale_id: i32, _product_id: i32) {
    let client: Client = {
        let client_ref = siv.user_data::<Client>().unwrap();
        client_ref.clone()
    };

    let async_view = AsyncView::new_with_bg_creator(
        siv,
        move || {
            let response = client.delete(format!("http://localhost/cancel-sale/id/{}", sale_id)).send();

            let message = match response {
                Ok(response) => {
                    let status = response.status();
                    if status == reqwest::StatusCode::GONE {
                        response.text().unwrap()
                    } else {
                        format!("Error {}: {}", status, response.text().unwrap())
                    }
                }
                Err(err) => {
                    format!("{:?}", err)
                }
            };

            Ok(message)
        },
        move |message: String| {
            Dialog::info(message)
        },
    ).max_width(150);

    siv.add_layer(async_view);
}