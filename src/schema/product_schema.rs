use diesel::table;

table! {
    product (id) {
        id -> Integer,
        name -> Varchar,
        category -> VarChar,
        quantity -> Integer,
        price -> Integer
    }
}