use crate::parser::types::LogCell;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::not_line_ending,
    combinator::{map, recognize},
    sequence::{delimited, tuple},
    IResult,
};

fn is_valid_unwrapped_cell_char(c: char) -> bool {
    let cv = c as u32;
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C) && (cv != 0x5D) && (cv != 0x2C) && (cv != 0x29)
}

pub fn is_valid_wrapped_cell_char(c: char) -> bool {
    let cv = c as u32;
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C)
}

/**
 * Wrapped cells are cells wrapped in quotations
 */
pub fn wrapped_cell(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("\""),
        take_while1(is_valid_wrapped_cell_char),
        tag("\""),
    )(input)
}

/**
 * Unwrapped cells are cell which have values such as SPELL_CAST
 */
pub fn unwrapped_cell(input: &str) -> IResult<&str, &str> {
    take_while1(is_valid_unwrapped_cell_char)(input)
}

/**
 * Emote cells are a special cell that is not fun to parse, we cheat here
 */
pub fn emote_cell(input: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("|T"), not_line_ending)))(input)
}

/**
 * String cells include unwrapped and wrapped cells
 */
pub fn string_cell(input: &str) -> IResult<&str, LogCell> {
    let parser = alt((emote_cell, wrapped_cell, unwrapped_cell));

    map(parser, |s| LogCell::Str(s))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emote_cell() {
        assert_eq!(emote_cell("|T"), Ok(("", "|T")));
        assert_eq!(emote_cell("|T its me mario"), Ok(("", "|T its me mario")))
    }

    #[test]
    fn test_unwrapped_cell() {
        assert_eq!(unwrapped_cell("hello1)"), Ok((")", "hello1")));
        assert_eq!(unwrapped_cell("hello1,"), Ok((",", "hello1")));
        assert_eq!(unwrapped_cell("SPELL_CAST"), Ok(("", "SPELL_CAST")));
    }

    #[test]
    fn test_wrapped_cell() {
        assert_eq!(wrapped_cell("\"hey\""), Ok(("", "hey")));
    }
}
