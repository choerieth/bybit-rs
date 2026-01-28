use crate::{models::BybitResponse, AnnouncementResult, Error};

pub struct BybitClient {
    base_url: String,
    agent: ureq::Agent,
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

    pub fn get_announcements(&self, params: AnnouncementParams) -> Result<AnnouncementResult, Error> {
        let url = format!("{}/v5/announcements/index", self.base_url.trim_end_matches('/'));
        let mut req = self.agent.get(&url);
        for (key, value) in params.to_query() {
            req = req.query(&key, &value);
        }

        let response = req.call().map_err(Error::Http)?;
        let reader = response.into_reader();
        let api: BybitResponse<AnnouncementResult> = serde_json::from_reader(reader).map_err(Error::Json)?;

        if api.ret_code != 0 {
            return Err(Error::Api {
                code: api.ret_code,
                msg: api.ret_msg,
            });
        }

        api.result.ok_or(Error::MissingField("result"))
    }
}

#[derive(Debug, Clone)]
pub struct AnnouncementParams {
    pub locale: String,
    pub type_key: Option<String>,
    pub tag: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl AnnouncementParams {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            locale: locale.into(),
            type_key: None,
            tag: None,
            page: None,
            limit: None,
        }
    }

    pub fn to_query(&self) -> Vec<(String, String)> {
        let mut query = Vec::with_capacity(5);
        query.push(("locale".to_string(), self.locale.clone()));
        if let Some(value) = &self.type_key {
            query.push(("type".to_string(), value.clone()));
        }
        if let Some(value) = &self.tag {
            query.push(("tag".to_string(), value.clone()));
        }
        if let Some(value) = self.page {
            query.push(("page".to_string(), value.to_string()));
        }
        if let Some(value) = self.limit {
            query.push(("limit".to_string(), value.to_string()));
        }
        query
    }
}
