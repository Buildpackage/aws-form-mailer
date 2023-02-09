
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
    fn low(&self) -> f32;
    fn volume(&self) -> f32;*/

}

impl StockInfo {

    /* functions for returning to console stockInfo */
}

#[async_trait]
impl Stock for StockInfo {

    async fn new(symbol: String) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let data = reqwest::get(url(symbol.to_string())).await.unwrap().text().await.unwrap();
        let mut open: f64 = 0.0;
        let mut high: f64 = 0.0;
        let mut low: f64 = 0.0;
        let mut close: f64 = 0.0;
        let mut volume: f64 = 0.0;
        let mut symbol: String = "Cannot parse error".to_string();
        let mut exchange: String = "Cannot parse error".to_string();
        let mut date: String = "Cannot parse error".to_string();


        let v: Value = serde_json::from_str(&data)?;
        for (key, value) in v.as_object().unwrap() {
            if key == "data"{
                for item in value.as_array().unwrap(){
                    for (key, value) in item.as_object().unwrap() {
                        match key.as_str() {
                            "open" => open = value.as_f64().unwrap(),
                            "high" => high = value.as_f64().unwrap(),
                            "low" => low = value.as_f64().unwrap(),
                            "close" => close = value.as_f64().unwrap(),
                            "volume"=> volume = value.as_f64().unwrap(),
                            "symbol"=> symbol = value.to_string(),
                            "exchange" => exchange = value.to_string(),
                            "date" => date = value.to_string(),
                            _ => ()
                        }
                    }
                }