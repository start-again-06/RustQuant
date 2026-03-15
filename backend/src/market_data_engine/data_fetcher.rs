#[derive(Debug, Clone)]
pub struct MarketPrice {

    pub symbol: String,
    pub price: f64,
}

pub fn fetch_market_prices() -> Vec<MarketPrice> {

    // placeholder data
    vec![
        MarketPrice {
            symbol: "AAPL".to_string(),
            price: 185.2,
        },
        MarketPrice {
            symbol: "TSLA".to_string(),
            price: 245.8,
        },
        MarketPrice {
            symbol: "MSFT".to_string(),
            price: 310.4,
        },
    ]
}
