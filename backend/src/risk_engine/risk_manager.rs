use std::collections::HashMap;

use super::position_limits;

pub fn validate_trade(
    holdings: &HashMap<String, i32>,
    symbol: &str,
    quantity: i32,
) -> bool {

    let max_position = 100;

    position_limits::check_position_limit(
        holdings,
        symbol,
        quantity,
        max_position,
    )
}
