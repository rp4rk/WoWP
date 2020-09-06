mod parser;

use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::{map, opt},
  multi::separated_list,
  sequence::delimited,
  IResult,
};

use parser::{
  numbers::numbers::{hex_cell, number_cell},
  strings::strings::string_cell,
  types::LogCell,
};

// This is the largest unit of a cell
fn parse_log_cell(input: &str) -> IResult<&str, LogCell> {
  alt((
    square_grouped_cells,
    grouped_cells,
    hex_cell,
    number_cell,
    string_cell,
  ))(input)
}

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

fn parse_log_line(input: &str) -> IResult<&str, Vec<LogCell>> {
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

fn main() {
  let test_string = r#"EMOTE,Creature-0-3781-2164-22090-152853-000027F1FF,"Silivaz the Zealous",0000000000000000,nil,|TInterface\ICONS\SPELL_NATURE_EARTHQUAKE.BLP:20|t Silivaz is casting |cFFFF0000|Hspell:301807|h[Zealous Eruption]|h|r!"#;

  let res = parse_log_line(test_string);
  println!("{:?}", res);
}
