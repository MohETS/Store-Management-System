mod sale_schema;
mod product_schema;
mod sale_item_schema;
mod store_schema;
mod store_account_schema;

pub use sale_schema::sale;
pub use product_schema::product;
pub use sale_item_schema::sale_item;
pub use store_account_schema::store_account;


diesel::joinable!(sale_item -> sale (sale_id));
diesel::joinable!(sale_item -> product (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    sale_item,
    sale,
    product,
);