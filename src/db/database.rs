use std::env;
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn connect() -> Pool<Postgres>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect to database");

    pool

}