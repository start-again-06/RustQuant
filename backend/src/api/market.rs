use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct MarketData {

    pub symbol: String,
    pub price: f64,
}

pub async fn get_market_data() -> Json<Vec<MarketData>> {

    let data = vec![
        MarketData {
            symbol: "AAPL".to_string(),
            price: 185.2,
        },
        MarketData {
            symbol: "TSLA".to_string(),
            price: 245.8,
        },
    ];

    Json(data)
}
