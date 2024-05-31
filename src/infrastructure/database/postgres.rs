use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn init_postgres(connection_string: &str)
    -> Pool<Postgres>
{
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await
        .expect("Failed to create pool")
}