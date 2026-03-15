use crate::market_data_engine::data_fetcher;

pub fn get_market_prices() {

    let prices = data_fetcher::fetch_market_prices();

    println!("{:?}", prices);
}
