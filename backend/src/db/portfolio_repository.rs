use sqlx::{Pool, Postgres};

pub async fn get_portfolio(
    pool: &Pool<Postgres>,
    user_id: i32,
) {

    let rows = sqlx::query(
        "SELECT symbol, quantity FROM portfolio WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap();

    println!("{:?}", rows);
}
