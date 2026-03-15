use sqlx::{Pool, Postgres};

pub async fn save_trade(
    pool: &Pool<Postgres>,
    symbol: &str,
    quantity: i32,
    side: &str,
) {

    sqlx::query(
        "INSERT INTO trades (symbol, quantity, side) VALUES ($1, $2, $3)"
    )
    .bind(symbol)
    .bind(quantity)
    .bind(side)
    .execute(pool)
    .await
    .unwrap();
}
