use super::indicators;

#[derive(Debug)]
pub struct FeatureVector {

    pub moving_avg: f64,
    pub volatility: f64,
}

pub fn generate_features(prices: &[f64]) -> FeatureVector {

    let ma = indicators::moving_average(prices);
    let vol = indicators::volatility(prices);

    FeatureVector {
        moving_avg: ma,
        volatility: vol,
    }
}
