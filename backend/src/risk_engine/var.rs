pub fn calculate_var(prices: &[f64], confidence: f64) -> f64 {

    let mut returns = Vec::new();

    for i in 1..prices.len() {
        returns.push((prices[i] - prices[i - 1]) / prices[i - 1]);
    }

    returns.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = ((1.0 - confidence) * returns.len() as f64) as usize;

    returns[index]
}
