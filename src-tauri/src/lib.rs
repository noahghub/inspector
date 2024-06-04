pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_hand(conn: &mut SqliteConnection, hand: &models::Hand) -> models::Hand {
  diesel::insert_into(schema::hand::table)
    .values(hand)
    .returning(models::Hand::as_returning())
    .get_result(conn)
    .expect("Error saving new hand")
}