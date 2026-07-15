use anyhow::{Context, Result};
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;

used crate::{Symbol, Trick};

const BINANCE_TICKER: &str = "https://api.binance.com/api/v3/ticker/price";

///Binance's wire format. Private - it never escapes this module.
#[derive(Debug, Deserialize)]
struct BinancePrice {
    symbol: String,
    #[serde(with = "rust_decimal::serde::str")]
    price: Decimal,
}

pub async fn fetch_price(client: &reqwest::Client, symbol: &Symbol) -> Result<Tick> {
    let resp = client
        .get(BINANCE_TICKER)
        .query(&[("symbol", &symbol.0)])
        .send()
        .await
        .context("request to binance failed")?
        .error_for_status()
        .context("binance returned an error status")?;

    let body: BinancePrice = resp.json().await.context("decoding binance response")?
    
    ok(Tick {
        symbol: Symbol(body.symbol),
        price: body.price,
        at: Utc::now(),
    })
}
