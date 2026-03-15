pub fn moving_average(prices: &[f64]) -> f64 {

    let sum: f64 = prices.iter().sum();
    sum / prices.len() as f64
}

pub fn volatility(prices: &[f64]) -> f64 {

    let mean = moving_average(prices);

    let variance: f64 = prices
        .iter()
        .map(|p| (p - mean).powi(2))
        .sum::<f64>()
        / prices.len() as f64;

    variance.sqrt()
}
