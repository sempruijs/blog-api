use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::routes::*;
pub mod db;
pub mod handlers;

pub mod routes;

//TODO: make age u32
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct User {
    name: String,
    age: i32,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    serve_routes(pool).await;

    Ok(())
}
