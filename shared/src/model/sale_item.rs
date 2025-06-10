use crate::schema::{sale_item};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sale_item)]
pub struct SaleItem {
    pub id: i32,
    pub sale_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub product_price: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sale_item)]
pub struct NewSaleItem {
    pub sale_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub product_price: i32,
}


impl SaleItem {
    /*  UC-02 - Save Sale */
    pub fn create_sale_item(conn: &mut PgConnection, new_sale: &NewSaleItem) -> QueryResult<SaleItem> {
        diesel::insert_into(sale_item::table).values(new_sale).get_result(conn)
    }


    /*  UC-03 - Cancel Sale */
    pub fn delete_sale_item(conn: &mut PgConnection, sale_id: i32) -> QueryResult<usize> {
        diesel::delete(sale_item::table.find(sale_id)).execute(conn)
    }


    pub fn get_all_sale_items(conn: &mut PgConnection) -> QueryResult<Vec<SaleItem>> {
        sale_item::table.load::<SaleItem>(conn)
    }
}
