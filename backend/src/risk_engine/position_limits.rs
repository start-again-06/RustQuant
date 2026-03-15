use std::collections::HashMap;

pub fn check_position_limit(
    holdings: &HashMap<String, i32>,
    symbol: &str,
    quantity: i32,
    max_position: i32,
) -> bool {

    let current = holdings.get(symbol).unwrap_or(&0);

    current + quantity <= max_position
}
