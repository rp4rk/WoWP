use serde::{self, Serialize};

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogCell<'a> {
    Number(f64),
    Str(&'a str),
    Array(Vec<LogCell<'a>>),
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
