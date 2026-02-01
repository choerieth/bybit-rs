mod client;
mod models;

pub use client::{AnnouncementParams, BybitClient, SystemStatusParams, KlineParams};
pub use models::{
    Announcement, AnnouncementResult, AnnouncementType, SystemStatus, SystemStatusResult,
    MarketTimeResult, KlineResult,
};

#[derive(Debug)]
pub enum Error {
    Http(ureq::Error),
    Json(serde_json::Error),
    Api { code: i32, msg: String },
    MissingField(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(err) => write!(f, "http error: {err}"),
            Error::Json(err) => write!(f, "json error: {err}"),
            Error::Api { code, msg } => write!(f, "api error: {code} {msg}"),
            Error::MissingField(field) => write!(f, "missing field: {field}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Error::Http(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Json(value)
    }
}
