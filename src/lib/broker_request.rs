use super::broker_options::{Function, DataType};

pub struct BrokerRequest<'a>{
    pub function: Function<'a>,
    pub symbol: &'a str,
    pub api_key: &'a str,
    pub data_type: DataType
}
impl<'a> BrokerRequest<'a>{
    pub fn new(symbol: &'a str, api_key: &'a str, function: Function<'a>, data_type: DataType) -> BrokerRequest<'a>{
        BrokerRequest { function: function, symbol: symbol, api_key: api_key, data_type: data_type } 
    }

    pub fn get_url(&self) -> String {
        format!("https://alphavantage.co/query{}&datatype={}&symbol={}&apikey={}", self.function.get_val(), self.data_type.get_val(), self.symbol, self.api_key)
    }
}
