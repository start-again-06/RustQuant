use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Prediction {

    pub symbol: String,
    pub signal: String,
}

pub async fn get_prediction() -> Json<Prediction> {

    Json(Prediction {

        symbol: "AAPL".to_string(),
        signal: "BUY".to_string()
    })
}
