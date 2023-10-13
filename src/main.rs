use lib::{broker_request::BrokerRequest, broker_options::{Function, DataType, Interval, OutputSize}};
use reqwest::Error;

use crate::lib::broker_api::BrokerAPI;

pub mod lib;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let munehisa_api = BrokerAPI::new("SP1OPJVF3TXAKDWN");


    let func: Function = Function::TimeSeriesWeekly;//Function::TimeSeriesIntraday(Interval::OneMin, Some(true), Some(true), None, Some(OutputSize::Full));
    let data_type: DataType = DataType::JSON;
    let response = munehisa_api.request("AZO", func, data_type).await?;
    Ok(())
}
