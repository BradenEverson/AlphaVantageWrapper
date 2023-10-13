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
    pub fn clean_data(&self) -> Vec<TimeSeries> {
        let mut response_timeseries: Vec<TimeSeries> = vec![];

        for (_, data_entries) in self.time_series.iter() {
            for (time_stamp, data) in data_entries.entries.iter(){
                let high_f64: f64 = str::parse::<f64>(&data.high).unwrap();
                let low_f64: f64 = str::parse::<f64>(&data.low).unwrap();
                let close_f64: f64 = str::parse::<f64>(&data.close).unwrap();
                let open_f64: f64 = str::parse::<f64>(&data.open).unwrap();

                let volume_i32: i32 = str::parse::<i32>(&data.volume).unwrap();
                let ts: TimeSeries = TimeSeries::new(open_f64, close_f64, high_f64, low_f64, volume_i32, time_stamp);

                response_timeseries.push(ts);
            }
        }
        response_timeseries
    }
}
