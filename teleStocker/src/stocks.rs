use reqwest::Error;
use serde::{Deserialize, Serialize};

use stocker_traits::Stock;
// use stocker_traits_derive::Stock;


#[derive(Serialize, Deserialize, Debug)]
struct StockRealTimeData {
    code: String,
    timestamp: i64,
    gmtoffset: u8,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: u32,
    #[serde(rename = "previousClose")]
    previous_close: f32,
    change: f32,
}

impl StockRealTimeData {
    fn new(data: String) -> Self {
        let deserealized: Self = serde_json::from_str(&data).expect("Invalid data");
        deserealized
    }
}
impl Stock for StockRealTimeData{
    fn data(self) -> String {
        format!{"Open: {}\nHigh: {}\nLow{}\nClose: {}\nVolume: {}", self.open, self.high, self.low, self.close, self.volume}
    }
}


pub async fn get_stock() -> Result<impl Stock, Error> {
    let body: String = reqwest::get("https://eodhistoricaldata.com/api/real-time/AAPL.US?fmt=json&api_token=demo")
        .await?
        .text()
        .await?;
    let stock = StockRealTimeData::new(body);
    Ok(stock) 
}
