use bybit::BybitClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BybitClient::new();
    let result = client.get_market_time()?;

    println!("timeSecond: {}", result.time_second);
    println!("timeNano: {}", result.time_nano);

    Ok(())
}
