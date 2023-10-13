pub enum Interval{
    OneMin,
    FiveMin,
    FifteenMin,
    ThirtyMin,
    SixtyMin
}

impl Interval{
    pub fn get_val(&self) -> &str{
        return match self {
            Self::OneMin => "1min",
            Self::FiveMin => "5min",
            Self::FifteenMin => "15min",
            Self::ThirtyMin => "30min",
            Self::SixtyMin => "60min"
        }
    }
}

pub enum DataType{
    JSON,
    CSV
}
impl DataType{
    pub fn get_val(&self) -> &str{
        return match self{
            Self::JSON => "json",
            Self::CSV => "csv"
        }
    }
}

pub enum OutputSize{
    Compact,
    Full
}
impl OutputSize{
    pub fn get_val(&self) -> &str{
        return match self{
            Self::Compact => "compact",
            Self::Full => "full"
        }
    }
}

pub enum function<'a>{
    TimeSeriesIntraday(
        Interval,
        //adjusted
        Option<bool>,
        //extended_hours
        Option<bool>,
        //Month
        Option<&'a str>,
        Option<OutputSize>
    ),
    TimeSeriesDaily(
        Option<OutputSize>,
    ),
    TimeSeriesDailyAdjusted(
        Option<OutputSize>,
    ),
    TimeSeriesWeekly,
    TimeSeriesWeeklyAdjusted,
    TimeSeriesMonthly,
    TimeSeriesMonthlyAdjusted,
    QuoteEndpoint
}

//NOTE: Function enum handles all unique api parameters of each function, 
//still must implement apikey, symbol and datatype on the higher level
impl<'a> function<'a>{
    pub fn get_val(&self) -> String{
        return match self{
            Self::TimeSeriesIntraday(interval, adjusted, extended_hours, month, output_size) => {
                let mut api_query = format!("?function=TIME_SERIES_INTRADAY&interval={}", interval.get_val());
                if let Some(adj) = adjusted {
                    api_query += &format!("&adjusted={}", adj.to_string());
                }
                if let Some(ext) = extended_hours{
                    api_query += &format!("&extended_hours={}", ext.to_string());
                }
                if let Some(m) = month{
                    api_query += &format!("&month={}", m);
                }
                if let Some(os) = output_size {
                    api_query += &format!("&outputsize={}", os.get_val());
                }
                return api_query;
            },
            Self::TimeSeriesDaily(output_size) => {
                let mut api_query = format!("?function=TIME_SERIES_DAILY");
                if let Some(os) = output_size{
                    api_query += &format!("&outputsize={}", os.get_val());
                }
                return api_query;
            },
            Self::TimeSeriesDailyAdjusted(output_size) => {
                let mut api_query = format!("?function=TIME_SERIES_DAILY_ADJUSTED");
                if let Some(os) = output_size{
                    api_query += &format!("&outputsize={}", os.get_val());
                }
                return api_query;
            },
            Self::TimeSeriesWeekly => "?function=TIME_SERIES_WEEKLY".to_string(),
            Self::TimeSeriesWeeklyAdjusted => "?function=TIME_SERIES_WEEKLY_ADJUSTED".to_string(), 
            Self::TimeSeriesMonthly => "?function=TIME_SERIES_MONTHLY".to_string(),
            Self::TimeSeriesMonthlyAdjusted => "?function=TIME_SERIES_MONTHLY_ADJUSTED".to_string(),
            Self::QuoteEndpoint => "?function=GLOBAL_QUOTE".to_string()
        }
    }
}
