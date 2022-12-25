
use serde::{Serialize, Deserialize};
use std::env;
use dotenv::dotenv;
use std::error::Error;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StockInfo {