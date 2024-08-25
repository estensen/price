use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct CoinGeckoResponse {
    ethereum: Price,
}

#[derive(Deserialize)]
struct Price {
    usd: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";

    let response: CoinGeckoResponse = reqwest::get(url).await?.json().await?;

    println!("Current price of ETH is ${}", response.ethereum.usd);

    Ok(())
}
