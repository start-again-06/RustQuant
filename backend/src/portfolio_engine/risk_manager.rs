use super::portfolio_manager::Portfolio;

pub fn check_position_limit(
    portfolio: &Portfolio,
    symbol: &str,
    max_position: i32,
) -> bool {

    if let Some(pos) = portfolio.holdings.get(symbol) {

        *pos < max_position
    } else {

        true
    }
}
