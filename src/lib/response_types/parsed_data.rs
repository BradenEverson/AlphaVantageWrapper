use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeSeries{
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub vol: i32
}
