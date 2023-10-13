mod lib;

use crate::lib::broker_options::{Function, DataType, Interval, OutputSize};
use lib::broker_response::BrokerResponse;
use reqwest::Error;

use crate::lib::broker_api::BrokerAPI;

#[tokio::main]
async fn main() -> Result<(), Error>{
    Ok(())
}
