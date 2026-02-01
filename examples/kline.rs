use bybit::{BybitClient, KlineParams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BybitClient::new();
    let mut params = KlineParams::new("BTCUSDT", "60");
    params.category = Some("spot".to_string());
    params.start = Some(1700000000000i64);
    params.limit = Some(10);

    let result = client.get_kline(params)?;

    let category = result.category.as_deref().unwrap_or("unknown");
    let symbol = result.symbol.as_deref().unwrap_or("unknown");
    println!("{} {}", category, symbol);
    for candle in result.list {
        println!("{:?}", candle);
    }

    Ok(())
}
