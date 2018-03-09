#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;
extern crate chrono;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn staging_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn warehouse_connection() -> PgConnection {
    dotenv().ok();

    let database_url  = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let warehouse_url = database_url.replace("/Staging", "/presencebot");
    PgConnection::establish(&warehouse_url).expect(&format!("Error connecting to {}", warehouse_url))
}

