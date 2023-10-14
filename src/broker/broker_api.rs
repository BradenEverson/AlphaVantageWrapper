use crate::broker::{broker_response::{*}, response_types::api_data::{StockData}};
use reqwest::Error;
use super::{broker_request::BrokerRequest, broker_options::{Function, DataType}};

pub struct BrokerAPI<'a>{
    token: &'a str,
}

impl<'a> BrokerAPI<'a>{
    pub fn new(token: &str) -> BrokerAPI{
        BrokerAPI{
            token: token
        }
    }
    ///Sends a request to the AlphaVantage API and returns a vectorized list of stock data
    ///# Examples
    ///
    ///```
    ///let symbol = "AZO";
    ///let function = Function::TimeSeriesWeekly;
    ///let data_type = DataType::json;
    ///
    ///let api = BrokerAPI::new({API_KEY});
    ///
    ///let response = api.request(symbol, function, data_type);
    ///```
    pub async fn request(&self, symbol: &str, function: Function<'a>, data_type: DataType) -> Result<BrokerResponse, Error>{
        let request: BrokerRequest = BrokerRequest::new(symbol , self.token, function, data_type);
        let response = reqwest::get(request.get_url()).await?;
        let resp_json: StockData = response.json().await?;
        let resp_parse = resp_json.clean_data();
        Ok(BrokerResponse { time_series_data: resp_parse })
    }
}
