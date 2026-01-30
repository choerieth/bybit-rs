use crate::{models::BybitResponse, Error, MarketTimeResult};

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
}
