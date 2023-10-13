mod lib;

use lib::broker_api::BrokerAPI;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut api = BrokerAPI::new("key");
    Ok(())
}
