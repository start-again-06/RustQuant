use axum::{
    Router,
    routing::{get, post}
};

use crate::api::{
    health::health_check,
    auth::login,
    market::get_market_data,
    trading::place_trade,
    portfolio::get_portfolio,
    prediction::get_prediction
};

pub fn create_routes() -> Router {

    Router::new()

        // health
        .route("/health", get(health_check))

        // auth
        .route("/auth/login", post(login))

        // market
        .route("/market", get(get_market_data))

        // trading
        .route("/trade", post(place_trade))

        // portfolio
        .route("/portfolio", get(get_portfolio))

        // ML prediction
        .route("/predict", get(get_prediction))
}
