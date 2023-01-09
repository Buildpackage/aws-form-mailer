
use serde::{Serialize, Deserialize};
use std::env;
use dotenv::dotenv;
use std::error::Error;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StockInfo {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    symbol: String,
    exchange: String,
    date: String,
}

fn url(stock: String) -> String {
    dotenv().ok();
    let api_key = env::var("API_KEY")
        .expect("API_KEY must be set");
    let url: String = format!("https://api.marketstack.com/v1/eod/latest?access_key={KEY}&symbols={URL}", URL= stock, KEY= api_key);
    
    return url;
}
#[async_trait]
pub trait Stock {

    async fn new(symbol: String) -> Result<Self, Box<dyn Error>> where Self: Sized;
    /*fn open(&self) -> f32;
    fn high(&self) -> f32;
    fn close(&self) -> f32;