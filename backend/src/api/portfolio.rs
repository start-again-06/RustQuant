use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Holding {

    pub symbol: String,
    pub quantity: i32,
}

pub async fn get_portfolio() -> Json<Vec<Holding>> {

    let holdings = vec![
        Holding {
            symbol: "AAPL".to_string(),
            quantity: 10,
        },
        Holding {
            symbol: "TSLA".to_string(),
            quantity: 5,
        },
    ];

    Json(holdings)
}
