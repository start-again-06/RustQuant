use super::order_manager::Order;

pub fn execute_order(order: &Order) {

    println!(
        "Executing order: {} {} {}",
        order.side,
        order.quantity,
        order.symbol
    );

}
