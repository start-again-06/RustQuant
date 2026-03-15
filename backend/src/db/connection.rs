use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn establish_connection(database_url: &str) -> Pool<Postgres> {

    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database")
}
