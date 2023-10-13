use crate::lib::broker_response::{*};

pub struct BrokerAPI<'a>{
    token: &'a str,
}

impl BrokerAPI<'_>{
    pub fn new(token: &str) -> BrokerAPI{
        BrokerAPI{
            token: token
        }
    }
    pub fn request() -> BrokerResponse{
        BrokerResponse::new()
    }
}
