use super::portfolio_manager::Portfolio;

pub fn portfolio_value(
    portfolio: &Portfolio,
    prices: &std::collections::HashMap<String, f64>,
) -> f64 {

    let mut value = portfolio.cash;

    for (symbol, qty) in &portfolio.holdings {

        if let Some(price) = prices.get(symbol) {

            value += *qty as f64 * price;
        }
    }

    value
}
