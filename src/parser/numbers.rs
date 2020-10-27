use crate::parser::types::LogCell;
use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::{map, map_res},
    number::complete::double,
    sequence::tuple,
    IResult,
};
use std::num::ParseIntError;

// Parser for number cells
pub fn number_cell(input: &str) -> IResult<&str, LogCell> {
    map(double, LogCell::Number)(input)
}

fn hex_to_f64<'a>(n: (&str, &'a str)) -> Result<LogCell<'a>, ParseIntError> {
    let n = i64::from_str_radix(n.1, 16)?;

    Ok(LogCell::Number(n as f64))
}

// Parses hex numbers such as 0x8
pub fn hex_cell(input: &str) -> IResult<&str, LogCell> {
    let parser = tuple((tag("0x"), alphanumeric1));
    map_res(parser, hex_to_f64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::types::LogCell;

    #[test]
    fn test_number_cell() {
        assert_eq!(number_cell("3.14"), Ok(("", LogCell::Number(3.14))));
        assert_eq!(number_cell("3"), Ok(("", LogCell::Number(3.0))));
        assert_eq!(number_cell("0.14"), Ok(("", LogCell::Number(0.14))));
        assert_eq!(number_cell("-0.14"), Ok(("", LogCell::Number(-0.14))));
    }

    #[test]
    fn text_hex_cell() {
        assert_eq!(hex_cell("0x8"), Ok(("", LogCell::Number(8.0))));
        assert_eq!(hex_cell("0x511"), Ok(("", LogCell::Number(1297.0))));
        assert_eq!(hex_cell("0xa48"), Ok(("", LogCell::Number(2632.0))));
    }
}
