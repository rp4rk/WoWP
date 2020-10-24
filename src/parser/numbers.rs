use crate::parser::types::LogCell;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1},
    combinator::{map, opt, recognize},
    sequence::{pair, tuple},
    IResult,
};

// Parser for fractions, e.g. .34
fn frac(input: &str) -> IResult<&str, &str> {
    recognize(pair(tag("."), digit1))(input)
}

// Parser for number cells
pub fn number_cell(input: &str) -> IResult<&str, LogCell> {
    let parser = recognize(tuple((opt(tag("-")), digit1, opt(frac))));
    map(parser, |s| {
        let n = s.parse().unwrap();
        LogCell::Number(n)
    })(input)
}

// Parses hex numbers such as 0x8
pub fn hex_cell(input: &str) -> IResult<&str, LogCell> {
    let parser = tuple((tag("0x"), alphanumeric1));
    map(parser, |s| {
        let n = i64::from_str_radix(s.1, 16);

        match n {
            Ok(v) => LogCell::Number(v as f64),
            Err(e) => panic!(e),
        }
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::types::LogCell;

    #[test]
    fn test_frac() {
        assert_eq!(frac(".34"), Ok(("", ".34")))
    }

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
