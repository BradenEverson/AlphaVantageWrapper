use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeSeries{
    pub time_stamp: String,

    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub vol: i32
}

impl TimeSeries{
    pub fn new(open: f64, close: f64, high: f64, low: f64, vol: i32, time_stamp: &str) -> TimeSeries{
        TimeSeries { time_stamp: time_stamp.to_string(), open: open, close: close, high: high, low: low, vol: vol }
    }
}
