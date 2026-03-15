use std::collections::HashMap;

#[derive(Debug)]
pub struct Portfolio {

    pub cash: f64,
    pub holdings: HashMap<String, i32>,
}

impl Portfolio {

    pub fn new(initial_cash: f64) -> Self {

        Portfolio {
            cash: initial_cash,
            holdings: HashMap::new(),
        }
    }

    pub fn buy(&mut self, symbol: String, quantity: i32, price: f64) {

        let cost = price * quantity as f64;

        if self.cash >= cost {

            self.cash -= cost;

            let entry = self.holdings.entry(symbol).or_insert(0);
            *entry += quantity;
        }
    }

    pub fn sell(&mut self, symbol: String, quantity: i32, price: f64) {

        if let Some(pos) = self.holdings.get_mut(&symbol) {

            if *pos >= quantity {

                *pos -= quantity;

                self.cash += price * quantity as f64;
            }
        }
    }
}
