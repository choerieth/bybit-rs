use bybit::{BybitClient, SystemStatusParams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BybitClient::new();
    let params = SystemStatusParams::default();
    let result = client.get_system_status(params)?;

    println!("system status entries: {}", result.list.len());
    for status in result.list {
        println!(
            "{} | {} | {} -> {} | env={} | maintainType={}",
            status.id,
            status.title,
            status.begin,
            status.end,
            status.env,
            status.maintain_type
        );
    }

    Ok(())
}
