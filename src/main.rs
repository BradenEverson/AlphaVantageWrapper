mod lib;

use crate::lib::broker_options::{Function, DataType, Interval, OutputSize};
use reqwest::Error;

use crate::lib::broker_api::BrokerAPI;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let munehisa_api = BrokerAPI::new("SP1OPJVF3TXAKDWN");


    let func: Function = Function::TimeSeriesIntraday(Interval::OneMin, Some(true), Some(true), None, None);
    let data_type: DataType = DataType::JSON;
    let _response = munehisa_api.request("AZO", func, data_type).await?;
    Ok(())
}
