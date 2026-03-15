use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TradeRequest {

    pub symbol: String,
    pub quantity: i32,
    pub side: String
}

#[derive(Serialize)]
pub struct TradeResponse {

    pub status: String
}

pub async fn place_trade(
    Json(payload): Json<TradeRequest>
) -> Json<TradeResponse> {

    println!(
        "Trade: {} {} {}",
        payload.side,
        payload.quantity,
        payload.symbol
    );

    Json(TradeResponse {
        status: "order placed".to_string()
    })
}
