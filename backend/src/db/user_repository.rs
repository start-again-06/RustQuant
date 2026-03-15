use sqlx::{Pool, Postgres};

pub async fn create_user(
    pool: &Pool<Postgres>,
    username: &str,
    password: &str,
) {

    sqlx::query(
        "INSERT INTO users (username, password) VALUES ($1, $2)"
    )
    .bind(username)
    .bind(password)
    .execute(pool)
    .await
    .unwrap();
}
