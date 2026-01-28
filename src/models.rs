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
