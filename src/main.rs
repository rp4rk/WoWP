mod matcher;
mod parser;

use linereader::LineReader;
use matcher::loader::{create_hashmap, load_templates, map_to_json};
use parser::types::LogCell;
use serde_json::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::str::from_utf8;

fn create_json_representation(eventMap: &Value, event: Vec<LogCell>) {}

fn read_lines(map: HashMap<String, Value>) {
    let file = File::open("WoWCombatLog.txt").expect("open");

    let mut reader = LineReader::new(file);

    while let Some(line) = reader.next_line() {
        let line = line.expect("Read error");
        let string = from_utf8(line).expect("Parse error");
        let test = parser::parse_log_line(string);

        if let LogCell::Str(v) = test.1[0] {
            let event_map_result = map.get(v);

            match event_map_result {
                Some(event_type) => create_json_representation(event_type, test.1),
                None => println!("Unhandled event {}", v),
            }
        }
    }
}

fn main() -> Result<()> {
    let res = load_templates(Path::new("./src/event_structures")).unwrap();
    let json = map_to_json(res);

    let map = create_hashmap(json);

    read_lines(map);

    Ok(())
}
