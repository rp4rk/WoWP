use serde::{self, Serialize, Serializer};

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
