use diesel::table;

table! {
    sale (id) {
        id -> Integer,
        total_price -> Integer,
    }
}