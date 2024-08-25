use serde::Deserialize;

#[derive(Deserialize)]
struct CoinGeckoResponse {
    ethereum: Price,
}

#[derive(Deserialize)]
struct Price {
    usd: f64,
}

async fn get_eth_price(url: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let response: CoinGeckoResponse = reqwest::get(url).await?.json().await?;
    Ok(response.ethereum.usd)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
    let price = get_eth_price(url).await?;
    println!("Current price of ETH is ${}", price);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_get_eth_price() {
        let mut server = Server::new_async().await;

        let _m = server
            .mock("GET", "/api/v3/simple/price")
            .match_query("ids=ethereum&vs_currencies=usd")
            .with_status(200)
            .with_body(r#"{"ethereum":{"usd":4000.0}}"#) // Corrected JSON format
            .create_async()
            .await;

        let url = &server.url();
        let full_url = format!("{}/api/v3/simple/price?ids=ethereum&vs_currencies=usd", url);

        let price = get_eth_price(&full_url).await.unwrap();
        assert_eq!(price, 4000.0);
    }
}
