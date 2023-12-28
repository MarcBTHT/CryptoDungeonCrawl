use reqwest;
use serde::Deserialize;

pub struct BitcoinData {
    pub price: f64,
}

#[derive(Deserialize)]
struct ApiResponse {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    item: Item,
}

#[derive(Deserialize)]
struct Item {
    latestRate: LatestRate,
}

#[derive(Deserialize)]
struct LatestRate {
    amount: String,
}

pub async fn fetch_data() -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://rest.cryptoapis.io/v2/market-data/assets/BTC";
    let api_key = "41b0c5757fe8400fad98a8c294d0478fe5faa1b2";
    let context = "yourExampleString";

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .query(&[("context", context)])
        .header("Content-Type", "application/json")
        .header("X-API-Key", api_key)
        .send()
        .await?;

    let body = response.text().await?;
    let api_response: ApiResponse = serde_json::from_str(&body)?;
    
    let latest_rate = api_response.data.item.latestRate;

    //println!("Bitcoin Price: {}", bitcoin_price);
    let bitcoin_price = latest_rate.amount.parse::<f64>()?;

    Ok(bitcoin_price)
}

