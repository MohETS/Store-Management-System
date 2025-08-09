use crate::schema::store_account;
use diesel::{PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, JsonSchema, Debug)]
#[diesel(table_name = store_account)]
pub struct StoreAccount {
    pub username: String,
    pub password: String,
}

impl StoreAccount {
    pub fn login_authentification(conn: &mut PgConnection, username: &String, password: &String) -> bool {
        match store_account::table.find(username).first::<StoreAccount>(conn) {
            Ok(account) => {
                let account:StoreAccount = account.into();
                Self::verify_password(account, password)
            }

            Err(diesel::result::Error::NotFound) => { false }

            _ => {false}
        }

    }

    fn verify_password(account: StoreAccount, password: &String) -> bool {
        account.password.eq(password)
    }
}