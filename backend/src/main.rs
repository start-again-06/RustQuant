use axum::Router;

mod routes;
mod config;

mod api {
    pub mod health;
    pub mod auth;
    pub mod market;
    pub mod trading;
    pub mod portfolio;
    pub mod prediction;
}

mod utils {
    pub mod logger;
    pub mod error_handler;
}

mod market_data_engine {
    pub mod data_fetcher;
    pub mod indicators;
    pub mod feature_engineering;
}

mod trading_engine {
    pub mod signal_generator;
    pub mod order_manager;
    pub mod execution_engine;
}

mod portfolio_engine {
    pub mod portfolio_manager;
    pub mod risk_manager;
    pub mod performance;
}

mod risk_engine {
    pub mod risk_manager;
    pub mod position_limits;
    pub mod var;
    pub mod drawdown;
}

mod backtesting_engine {
    pub mod simulator;
    pub mod strategy_runner;
    pub mod performance_metrics;
}

mod services {
    pub mod auth_service;
    pub mod market_service;
    pub mod trading_service;
    pub mod portfolio_service;
    pub mod prediction_service;
}

mod db {
    pub mod connection;
    pub mod user_repository;
    pub mod trade_repository;
    pub mod portfolio_repository;
}

#[tokio::main]
async fn main() {

    utils::logger::init_logger();

    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind server");

    println!("RustQuantAI backend running on port 3000");

    axum::serve(listener, app).await.unwrap();
}
