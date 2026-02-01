use bybit::{AnnouncementParams, BybitClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BybitClient::new();
    let params = AnnouncementParams::new("en-US");
    let result = client.get_announcements(params)?;

    println!("total: {}", result.total);
    for item in result.list {
        println!("{} | {}", item.title, item.url);
    }

    Ok(())
}
