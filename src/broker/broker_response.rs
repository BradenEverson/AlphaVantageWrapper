use crate::lib::response_types::parsed_data::TimeSeries;

pub struct BrokerResponse{
    pub TimeSeriesData: Vec<TimeSeries>
}

impl BrokerResponse{
    pub fn new() -> BrokerResponse{
        BrokerResponse{TimeSeriesData: vec![]}
    }
}
