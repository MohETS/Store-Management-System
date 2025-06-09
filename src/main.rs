use diesel::{Connection, PgConnection};
use register::Register;

#[cfg(test)]
mod main_test;
mod register;
mod schema;
mod model;

fn main() {
    let database_url: &str = "postgres://postgres:mohets@localhost:5432/MohSounds";
    let conn = PgConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let mut siv = cursive::default();
    let mut register = Register::new(conn);

    register.setup_ui(&mut siv);
    siv.run();
}








