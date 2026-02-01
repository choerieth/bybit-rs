mod announcements;
mod system;
mod market;

pub use announcements::AnnouncementParams;
pub use system::SystemStatusParams;
pub use market::KlineParams;

pub struct BybitClient {
    pub(crate) base_url: String,
    pub(crate) agent: ureq::Agent,
}

impl BybitClient {
    pub fn new() -> Self {
        Self::with_base_url("https://api.bybit.com")
    }

    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            agent: ureq::Agent::new(),
        }
    }
}
