use crate::model::sale_item::{NewSaleItem, SaleItem};
use crate::model::Product;
use crate::schema::{product, sale};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = sale)]
pub struct Sale {
    pub id: i32,
    pub total_price: i32,
}


#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = sale)]
pub struct NewSale {
    pub total_price: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MakeSaleRequest {
    pub product_id: i32,
    pub quantity_sold: i32,
}

impl Sale {
    /*  UC-02 - Save Sale */
    pub fn create_sale(conn: &mut PgConnection, new_sale: &NewSale) -> QueryResult<Sale> {
        diesel::insert_into(sale::table).values(new_sale).get_result(conn)
    }

    // pub fn update_product_quantity(conn: &mut PgConnection, product_id: i32, new_quantity: i32) -> QueryResult<usize> {
    //     diesel::update(product::table.find(product_id)).set(product::quantity.eq(new_quantity)).execute(conn)
    // }

    pub fn make_sale(conn: &mut PgConnection, product_id: i32, quantity_sold: i32) -> QueryResult<(Sale, Product)> {
        let product_item = product::table
            .find(product_id)
            .first::<Product>(conn)?;

        if product_item.quantity < quantity_sold {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        let total_price = product_item.price * quantity_sold;

        let new_sale = NewSale {
            total_price,
        };

        let created_sale = Self::create_sale(conn, &new_sale)?;

        let new_sale_item = NewSaleItem {
            sale_id: created_sale.id,
            product_id,
            quantity: quantity_sold,
            product_price: product_item.price,
        };

        SaleItem::create_sale_item(conn, &new_sale_item).expect("Sale item creation failed");

        let new_quantity = product_item.quantity - quantity_sold;
        let updated_product = diesel::update(product::table.find(product_id))
            .set(product::quantity.eq(new_quantity))
            .get_result::<Product>(conn)?;

        Ok((created_sale, updated_product))
    }


    /*  UC-03 - Cancel Sale */
    pub fn cancel_sale(conn: &mut PgConnection, sale_id: i32, _product_id: i32) -> QueryResult<usize> {
        // let sale_item = match Self::get_sale_by_id(conn, sale_id) {
        //     Ok(sale_item) => sale_item,
        //     Err(_) => return Ok(0),
        // };
        //
        // let current_product = match Product::search_product_by_id(conn, product_id) {
        //     Ok(current_product) => current_product,
        //     Err(_) => return Ok(0),
        // };
        // let restored_quantity = current_product.quantity + sale_item.product_amount;
        //
        // Self::update_product_quantity(conn, product_id, restored_quantity)?;

        SaleItem::delete_sale_item(conn, sale_id).expect("Sale item deletion failed");

        Self::delete_sale(conn, sale_id)
    }

    pub fn delete_sale(conn: &mut PgConnection, sale_id: i32) -> QueryResult<usize> {
        diesel::delete(sale::table.find(sale_id)).execute(conn)
    }

    // pub fn get_sale_by_id(conn: &mut PgConnection, sale_id: i32) -> QueryResult<Sale> {
    //     sale::table.find(sale_id).first::<Sale>(conn)
    // }


    pub fn get_all_sales(conn: &mut PgConnection) -> QueryResult<Vec<Sale>> {
        sale::table.load::<Sale>(conn)
    }
}