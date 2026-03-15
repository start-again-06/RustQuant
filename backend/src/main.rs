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
