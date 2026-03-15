use crate::trading_engine::order_manager;
use crate::trading_engine::execution_engine;

pub fn execute_trade(symbol: String, quantity: i32, side: String) {

    let order = order_manager::create_order(symbol, quantity, side);

    execution_engine::execute_order(&order);
}
