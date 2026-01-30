use crate::{models::BybitResponse, Error, SystemStatusResult};

use super::BybitClient;

impl BybitClient {
    pub fn get_system_status(&self, params: SystemStatusParams) -> Result<SystemStatusResult, Error> {
        let url = format!("{}/v5/system/status", self.base_url.trim_end_matches('/'));
        let mut req = self.agent.get(&url);
        for (key, value) in params.to_query() {
            req = req.query(&key, &value);
        }

        let response = req.call().map_err(Error::Http)?;
        let reader = response.into_reader();
        let api: BybitResponse<SystemStatusResult> =
            serde_json::from_reader(reader).map_err(Error::Json)?;

        if api.ret_code != 0 {
            return Err(Error::Api {
                code: api.ret_code,
                msg: api.ret_msg,
            });
        }

        api.result.ok_or(Error::MissingField("result"))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemStatusParams {
    pub id: Option<String>,
    pub state: Option<String>,
}

impl SystemStatusParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_query(&self) -> Vec<(String, String)> {
        let mut query = Vec::with_capacity(2);
        if let Some(value) = &self.id {
            query.push(("id".to_string(), value.clone()));
        }
        if let Some(value) = &self.state {
            query.push(("state".to_string(), value.clone()));
        }
        query
    }
}
