use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use std::env::var;

/// Create a connection to the database
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
