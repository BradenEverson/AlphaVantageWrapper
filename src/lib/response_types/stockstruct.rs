use serde::{Deserialize, Serialize};
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
pub struct TimeSeries {
    #[serde(flatten)]
    time_series: BTreeMap<String, MinuteData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MinuteData {
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
pub struct StockData {
    #[serde(rename = "Meta Data")]
    meta_data: MetaData,
    #[serde(alias = "Time Series (1min)", alias = "Weekly Time Series", alias = "Daily Time Series")]
    time_series: Option<TimeSeries>,
}
