use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeSeries<'a>{
    pub time_stamp: &'a str,

    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub vol: i32
}

impl<'a> TimeSeries<'a>{
    pub fn new(open: f64, close: f64, high: f64, low: f64, vol: i32, time_stamp: &str) -> TimeSeries<'a>{
        TimeSeries { time_stamp: time_stamp, open: open, close: close, high: high, low: low, vol: vol }
    }
}
