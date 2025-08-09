use diesel::table;

table! {
    store_account (username) {
        username -> Varchar,
        password -> VarChar,
    }
}