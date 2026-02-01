use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnnouncementResult {
    pub total: u64,
    pub list: Vec<Announcement>,
}

#[derive(Debug, Deserialize)]
pub struct Announcement {
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_info: AnnouncementType,
    pub tags: Vec<String>,
    pub url: String,
    #[serde(rename = "dateTimestamp")]
    pub date_timestamp: Option<i64>,
    #[serde(rename = "startDateTimestamp")]
    pub start_date_timestamp: Option<i64>,
    #[serde(rename = "endDateTimestamp")]
    pub end_date_timestamp: Option<i64>,
    #[serde(rename = "publishTime")]
    pub publish_time: Option<i64>,
}

// Announcements include a nested "type" object, so we model it separately for clarity.
#[derive(Debug, Deserialize)]
pub struct AnnouncementType {
    pub title: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct BybitResponse<T> {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct SystemStatusResult {
    pub list: Vec<SystemStatus>,
}

#[derive(Debug, Deserialize)]
pub struct SystemStatus {
    pub id: String,
    pub title: String,
    pub state: String,
    pub begin: String,
    pub end: String,
    pub href: String,
    #[serde(rename = "serviceTypes")]
    pub service_types: Vec<i32>,
    pub product: Vec<i32>,
    #[serde(rename = "uidSuffix")]
    pub uid_suffix: Vec<i32>,
    #[serde(rename = "maintainType")]
    pub maintain_type: i32,
    pub env: i32,
}

#[derive(Debug, Deserialize)]
pub struct MarketTimeResult {
    #[serde(rename = "timeSecond")]
    pub time_second: String,
    #[serde(rename = "timeNano")]
    pub time_nano: String,
}

#[derive(Debug, Deserialize)]
pub struct KlineResult {
    pub category: Option<String>,
    pub symbol: Option<String>,
    pub list: Vec<Vec<String>>,
}
