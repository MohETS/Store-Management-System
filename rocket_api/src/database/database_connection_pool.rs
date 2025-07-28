use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_database_connection_pool() -> DbPool {
    let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    let server = std::env::var("POSTGRES_SERVER").expect("POSTGRES_SERVER not set");
    let port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT not set");
    let db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB not set");

    let database_url: &str = &format!("postgres://{}:{}@{}:{}/{}", user, password, server, port, db);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool")
}