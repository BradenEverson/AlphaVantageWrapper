use crate::broker::response_types::parsed_data::TimeSeries;

pub struct BrokerResponse{
    pub time_series_data: Vec<TimeSeries>
}

impl BrokerResponse{
    pub fn new() -> BrokerResponse{
        BrokerResponse{time_series_data: vec![]}
    }
}
