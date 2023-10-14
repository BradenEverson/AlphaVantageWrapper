

use crate::lib::{broker_response::{*}, response_types::api_data::{StockData}};
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
    pub async fn request(&self, symbol: &str, function: Function<'a>, data_type: DataType) -> Result<BrokerResponse, Error>{
        let request: BrokerRequest = BrokerRequest::new(symbol , self.token, function, data_type);
        let response = reqwest::get(request.get_url()).await?;
        let resp_json: StockData = response.json().await?;
        println!("{:?}", resp_json); 
        Ok(BrokerResponse::new())
    }
}
