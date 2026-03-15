pub fn mean(values: &[f64]) -> f64 {

    values.iter().sum::<f64>() / values.len() as f64
}

pub fn std_dev(values: &[f64]) -> f64 {

    let mean = mean(values);

    let variance: f64 = values
        .iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;

    variance.sqrt()
}
