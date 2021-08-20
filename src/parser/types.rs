use chrono::prelude::*;
use serde::{self, Serialize, Serializer};
use std::fmt;

// This prevents floats with no remainder from displaying it
fn integer_serialize<S>(x: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let has_remainder = x.fract() != 0.0;

    match has_remainder {
        true => s.serialize_f64(*x),
        false => s.serialize_i64(*x as i64),
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogCell<'a> {
    #[serde(serialize_with = "integer_serialize")]
    Number(f64),
    Str(&'a str),
    Array(Vec<LogCell<'a>>),
    ComboPointSpender(ComboPointSpender),
}
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboPointSpender {
    pub energy: f64,
    pub combo_points: f64,
}

#[derive(Debug, PartialEq)]
pub struct LogEventDateTime<'a> {
    // The month an event occurred
    pub month: &'a str,
    // The day of the month an event occurred
    pub day: &'a str,
    // The hour an event occured
    pub hour: &'a str,
    // The minute an event occured
    pub minute: &'a str,
    // The second event occured
    pub second: &'a str,
    // The millisecond event occured
    pub ms: &'a str,
}

impl fmt::Display for LogEventDateTime<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let year = Local::now().year();
        let month = self.month.parse::<u32>().unwrap();
        let day = self.day.parse::<u32>().unwrap();
        let hour = self.hour.parse::<u32>().unwrap();
        let minute = self.minute.parse::<u32>().unwrap();
        let second = self.second.parse::<u32>().unwrap();
        let ms = self.ms.parse::<u32>().unwrap();
        let date_time =
            NaiveDate::from_ymd(year, month, day).and_hms_milli(hour, minute, second, ms);

        write!(f, "{}", date_time.to_string())
    }
}
