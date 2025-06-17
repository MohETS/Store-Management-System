use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_database_connection_pool() -> DbPool {
    let database_url: &str = "postgres://postgres:mohets@localhost:5432/MohStore";

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool")
}