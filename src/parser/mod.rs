pub mod cells;
pub mod dates;
pub mod numbers;
pub mod strings;
pub mod types;

use cells::parse_log_csv;
use dates::parse_date_time;
use types::{LogCell, LogEventDateTime};

use nom::{bytes::complete::tag, sequence::separated_pair};

pub fn parse_log_line(input: &str) -> (LogEventDateTime, Vec<LogCell>) {
    let res = separated_pair(parse_date_time, tag("  "), parse_log_csv)(input);
    let (_, result) = res.unwrap();

    return result;
}
