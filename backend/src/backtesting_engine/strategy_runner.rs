use crate::trading_engine::signal_generator::TradeSignal;

pub fn run_strategy(prices: &[f64]) -> Vec<TradeSignal> {

    let mut signals = Vec::new();

    for i in 1..prices.len() {

        if prices[i] > prices[i - 1] {

            signals.push(TradeSignal::Buy);

        } else {

            signals.push(TradeSignal::Sell);
        }
    }

    signals
}
