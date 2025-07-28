use crate::schema::product;
use diesel::{AsChangeset, PgConnection, PgTextExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, AsChangeset, Serialize, Deserialize, JsonSchema, Debug)]
#[diesel(table_name = product)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub quantity: i32,
    pub price: i32,
}


impl Product {
    /*  UC-01 - Select Product */
    pub fn search_product_by_id(conn: &mut PgConnection, product_id: i32) -> QueryResult<Product> {
        product::table.find(product_id).first::<Product>(conn)
    }


    pub fn search_product_by_name(conn: &mut PgConnection, product_name: &str) -> QueryResult<Vec<Product>> {
        let pattern = format!("%{}%", product_name);
        product::table.filter(product::name.ilike(pattern)).load::<Product>(conn)
    }

    pub fn search_product_by_category(conn: &mut PgConnection, product_category: &str) -> QueryResult<Vec<Product>> {
        let pattern = format!("%{}%", product_category);
        product::table.filter(product::category.ilike(pattern)).load::<Product>(conn)
    }


    /*  UC-04 - View Product Stock */
    pub fn get_all_products(conn: &mut PgConnection) -> QueryResult<Vec<Product>> {
        product::table.load::<Product>(conn)
    }

    pub fn update_product(conn: &mut PgConnection, product: Product) -> QueryResult<usize> {
        diesel::update(product::table.find(product.id)).set(product).execute(conn)
    }
}

