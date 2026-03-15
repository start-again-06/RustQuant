use crate::market_data_engine::feature_engineering::FeatureVector;

#[derive(Debug)]
pub enum TradeSignal {
    Buy,
    Sell,
    Hold,
}

pub fn generate_signal(features: &FeatureVector) -> TradeSignal {

    if features.moving_avg > 180.0 && features.volatility < 5.0 {
        TradeSignal::Buy
    } 
    else if features.volatility > 10.0 {
        TradeSignal::Sell
    } 
    else {
        TradeSignal::Hold
    }
}
