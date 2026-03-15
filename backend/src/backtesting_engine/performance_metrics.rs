pub fn calculate_returns(prices: &[f64]) -> f64 {

    let start = prices.first().unwrap();
    let end = prices.last().unwrap();

    (end - start) / start
}

pub fn sharpe_ratio(returns: &[f64]) -> f64 {

    let mean: f64 = returns.iter().sum::<f64>() / returns.len() as f64;

    let variance: f64 = returns
        .iter()
        .map(|r| (r - mean).powi(2))
        .sum::<f64>()
        / returns.len() as f64;

    let std_dev = variance.sqrt();

    if std_dev == 0.0 {
        0.0
    } else {
        mean / std_dev
    }
}
