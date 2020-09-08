use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list, sequence::separated_pair, IResult,
};

use crate::parser::types::LogEventDateTime;

fn parse_date(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(digit1, tag("/"), digit1)(input)
}

#[test]
fn test_parse_date() {
    let res = parse_date("10/17");

    assert_eq!(res, Ok(("", ("10", "17"))));
}

fn parse_time(input: &str) -> IResult<&str, Vec<&str>> {
    let delimiter_parser = alt((tag(":"), tag(".")));

    separated_list(delimiter_parser, digit1)(input)
}

#[test]
fn test_parse_time() {
    let res = parse_time("00:56:59.186");
    let res2 = parse_time("00:56:59.186 ");

    assert_eq!(res, Ok(("", vec!["00", "56", "59", "186"])));
    assert_eq!(res2, Ok((" ", vec!["00", "56", "59", "186"])));
}

pub fn parse_date_time<'a>(input: &'a str) -> IResult<&str, LogEventDateTime> {
    let parser = separated_pair(parse_date, tag(" "), parse_time);

    map(parser, |s| {
        let (date_data, time_data) = s;
        let (month_data, day_data) = date_data;

        LogEventDateTime {
            month: month_data,
            day: day_data,
            hour: time_data[0],
            minute: time_data[1],
            second: time_data[2],
            ms: time_data[3],
        }
    })(input)
}

#[test]
fn test_parse_date_time() {
    let res = parse_date_time("10/17 00:53:45.146");

    assert_eq!(
        res,
        Ok((
            "",
            LogEventDateTime {
                month: "10",
                day: "17",
                hour: "00",
                minute: "53",
                second: "45",
                ms: "146"
            }
        ))
    )
}
