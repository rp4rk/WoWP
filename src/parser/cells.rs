/**
* Parsing for specific types of cells present in the comma separated list
*/
use crate::parser::{
    numbers::{hex_cell, number_cell},
    strings::string_cell,
    types::LogCell,
};

use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list, sequence::delimited,
    IResult,
};

// Largest cell unit
pub fn parse_log_cell(input: &str) -> IResult<&str, LogCell> {
    alt((
        square_grouped_cells,
        grouped_cells,
        hex_cell,
        number_cell,
        string_cell,
    ))(input)
}

//
fn grouped_cells(input: &str) -> IResult<&str, LogCell> {
    let parser = delimited(tag("("), separated_list(tag(","), parse_log_cell), tag(")"));

    map(parser, |v| LogCell::Array(v))(input)
}

#[test]
fn test_grouped_cell() {
    assert_eq!(grouped_cells("()"), Ok(("", LogCell::Array(vec![]))));
    assert_eq!(
        grouped_cells("(hello,world)"),
        Ok((
            "",
            LogCell::Array(vec![LogCell::Str("hello"), LogCell::Str("world")])
        ))
    );
    assert_eq!(
        grouped_cells("(hello,\"world\")"),
        Ok((
            "",
            LogCell::Array(vec![LogCell::Str("hello"), LogCell::Str("world")])
        ))
    );
}

fn square_grouped_cells(input: &str) -> IResult<&str, LogCell> {
    let parser = delimited(tag("["), separated_list(tag(","), parse_log_cell), tag("]"));

    map(parser, |v| LogCell::Array(v))(input)
}

pub fn parse_log_csv(input: &str) -> IResult<&str, Vec<LogCell>> {
    separated_list(tag(","), parse_log_cell)(input)
}

#[test]
fn test_square_grouped_cell() {
    assert_eq!(
        square_grouped_cells("[hello,\"world\"]"),
        Ok((
            "",
            LogCell::Array(vec![LogCell::Str("hello"), LogCell::Str("world")])
        ))
    )
}
