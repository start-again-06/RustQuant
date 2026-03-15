pub fn calculate_drawdown(equity_curve: &[f64]) -> f64 {

    let mut peak = equity_curve[0];
    let mut max_drawdown = 0.0;

    for value in equity_curve {

        if *value > peak {
            peak = *value;
        }

        let drawdown = (peak - value) / peak;

        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
    }

    max_drawdown
}
