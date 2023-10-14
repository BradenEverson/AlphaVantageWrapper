use serde::{Deserialize, Serialize};
use crate::lib::response_types::parsed_data::TimeSeries;

use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    last_refreshed: Option<String>,
    #[serde(rename = "4. Interval")]
    interval: Option<String>,
    #[serde(rename = "5. Output Size")]
    output_size: Option<String>,
    #[serde(rename = "6. Time Zone")]
    time_zone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeriesData{
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeriesEntry {
    #[serde(flatten)]
    entries: BTreeMap<String, TimeSeriesData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockData {
    #[serde(rename="Meta Data")]
    meta_data: MetaData,
    #[serde(flatten)]
    time_series: BTreeMap<String, TimeSeriesEntry>,
}

impl StockData{
    pub fn clean_data(&mut self) -> Vec<TimeSeries> {
        let mut response_timeseries: Vec<TimeSeries> = vec![];
        
        for (time_stamp, data) in self.time_series.iter(){
        }

        response_timeseries
    }
}
