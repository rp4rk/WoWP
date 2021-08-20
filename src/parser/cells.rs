/**
* Parsing for specific types of cells present in the comma separated list
*/
use crate::parser::{
    numbers::{hex_cell, number_cell_master},
    strings::string_cell,
    types::LogCell,
};

use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::delimited, IResult,
};

// Largest cell unit
pub fn parse_log_cell(input: &str) -> IResult<&str, LogCell> {
    match &input[0..1] {
        "[" => square_grouped_cells(input),
        "(" => grouped_cells(input),
        "0" => match &input[0..2] {
            "0x" => hex_cell(input),
            _ => number_cell_master(input),
        },
        "1" => number_cell_master(input),
        "2" => number_cell_master(input),
        "3" => number_cell_master(input),
        "4" => number_cell_master(input),
        "5" => number_cell_master(input),
        "6" => number_cell_master(input),
        "7" => number_cell_master(input),
        "8" => number_cell_master(input),
        "9" => number_cell_master(input),
        "-" => number_cell_master(input),
        _ => string_cell(input),
    }
}

fn grouped_cells(input: &str) -> IResult<&str, LogCell> {
    let parser = delimited(
        tag("("),
        separated_list1(tag(","), parse_log_cell),
        tag(")"),
    );

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
    let parser = delimited(
        tag("["),
        separated_list1(tag(","), parse_log_cell),
        tag("]"),
    );

    map(parser, |v| LogCell::Array(v))(input)
}

pub fn parse_log_csv(input: &str) -> IResult<&str, Vec<LogCell>> {
    separated_list1(tag(","), parse_log_cell)(input)
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
