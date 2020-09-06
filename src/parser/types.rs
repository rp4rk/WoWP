#[derive(Debug, PartialEq)]
pub enum LogCell<'a> {
  Number(f64),
  Str(&'a str),
  Array(Vec<LogCell<'a>>),
}
