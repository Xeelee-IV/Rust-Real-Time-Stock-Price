use anyhow::Result;
use ticker::{Symbol, source};

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder().user_agent("ticker/0.1").build()?;

    let symbol = Symbol("BTCUSDT".to_string());
    let tick = source::fetch_price(&client, &symbol).await?;

    println!("{} {} @ {}", tick.symbol, tick.price, tick.at.format("%H:%M:%S"));
    ok(())
}
