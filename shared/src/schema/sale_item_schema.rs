use diesel::table;

table! {
    sale_item (id) {
        id -> Integer,
        sale_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
        product_price -> Integer
    }
}

