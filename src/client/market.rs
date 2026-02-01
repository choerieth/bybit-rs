use crate::{models::BybitResponse, Error, MarketTimeResult, KlineResult};

use super::BybitClient;

impl BybitClient {
    pub fn get_market_time(&self) -> Result<MarketTimeResult, Error> {
        let url = format!("{}/v5/market/time", self.base_url.trim_end_matches('/'));
        let response = self.agent.get(&url).call().map_err(Error::Http)?;
        let reader = response.into_reader();
        let api: BybitResponse<MarketTimeResult> =
            serde_json::from_reader(reader).map_err(Error::Json)?;

        if api.ret_code != 0 {
            return Err(Error::Api {
                code: api.ret_code,
                msg: api.ret_msg,
            });
        }

        api.result.ok_or(Error::MissingField("result"))
    }

    pub fn get_kline(&self, params: KlineParams) -> Result<KlineResult, Error> {
        let url = format!("{}/v5/market/kline", self.base_url.trim_end_matches('/'));
        let mut req = self.agent.get(&url);
        for (key, value) in params.to_query() {
            req = req.query(&key, &value);
        }

        let response = req.call().map_err(Error::Http)?;
        let reader = response.into_reader();
        let api: BybitResponse<KlineResult> = serde_json::from_reader(reader).map_err(Error::Json)?;

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
pub struct KlineParams {
    pub category: Option<String>,
    pub symbol: String,
    pub interval: String,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub limit: Option<u32>,
}

impl KlineParams {
    pub fn new(symbol: impl Into<String>, interval: impl Into<String>) -> Self {
        Self {
            category: None,
            symbol: symbol.into(),
            interval: interval.into(),
            start: None,
            end: None,
            limit: None,
        }
    }

    pub fn to_query(&self) -> Vec<(String, String)> {
        let mut query = Vec::with_capacity(6);
        if let Some(value) = &self.category {
            query.push(("category".to_string(), value.clone()));
        }
        query.push(("symbol".to_string(), self.symbol.clone()));
        query.push(("interval".to_string(), self.interval.clone()));
        if let Some(value) = self.start {
            query.push(("start".to_string(), value.to_string()));
        }
        if let Some(value) = self.end {
            query.push(("end".to_string(), value.to_string()));
        }
        if let Some(value) = self.limit {
            query.push(("limit".to_string(), value.to_string()));
        }
        query
    }
}
