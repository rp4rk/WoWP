use serde::{self, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogCell<'a> {
    Number(f64),
    Str(&'a str),
    Array(Vec<LogCell<'a>>),
}

// impl<'a> Display for LogCell<'a> {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         match *self {
//             LogCell::Str(v) => write!(f, v),
//             LogCell::Number(v) => write(!f, v),
//             LogCell::Array(v) => v.iter().map(|v| fmt),
//         }
//     }
// }

// impl Serialize for LogCell<LifeTime> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//     }
// }

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
