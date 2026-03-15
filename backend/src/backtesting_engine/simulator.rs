pub struct MarketSimulator {

    pub prices: Vec<f64>,
}

impl MarketSimulator {

    pub fn new(prices: Vec<f64>) -> Self {

        MarketSimulator { prices }
    }

    pub fn get_price(&self, index: usize) -> f64 {

        self.prices[index]
    }

    pub fn len(&self) -> usize {

        self.prices.len()
    }
}
