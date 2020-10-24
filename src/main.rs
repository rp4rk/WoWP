mod matcher;
mod parser;

use linereader::LineReader;
use matcher::loader::{create_hashmap, load_templates, map_to_json};
use parser::types::{LogCell, LogEventDateTime};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::str::from_utf8;

fn create_json_representation(
    event_map: &Value,
    event_type: String,
    time: LogEventDateTime,
    event: Vec<LogCell>,
) -> Result<String> {
    let event_map_obj = match event_map.as_object() {
        Some(v) => v,
        None => panic!("Could not determine an event map for some reason"),
    };

    let mut hmap = event_map_obj.into_iter().fold(HashMap::new(), |mut h, v| {
        {
            let event_value_index = match v.1.as_i64() {
                Some(v) => v as usize,
                None => panic!("Could not parse event value index {}", v.1),
            };

            let event_value = match event.get(event_value_index) {
                Some(v) => v,
                None => {
                    eprintln!(
                        "Failed to index {} properly with index {}",
                        event_type, event_value_index
                    );
                    eprintln!("{:?}", time);
                    eprintln!("{:?}", event);
                    return h;
                }
            };

            h.insert(v.0, event_value);
        }
        h
    });

    let key = &"eventType".to_string();
    let value = &LogCell::Str(event_type.as_str());
    hmap.insert(key, value);

    return serde_json::to_string_pretty(&hmap);
}

fn read_lines(map: HashMap<String, Value>) {
    let file = File::open("WoWCombatLog.txt").expect("open");
    let mut reader = LineReader::new(file);

    println!("[");
    while let Some(line) = reader.next_line() {
        let line = line.expect("Read error");
        let string = from_utf8(line).expect("Parse error");
        let parsed_line = parser::parse_log_line(string);

        if let LogCell::Str(v) = parsed_line.1[0] {
            let event_map_result = map.get(v);

            let v = match event_map_result {
                Some(event_type) => create_json_representation(
                    event_type,
                    v.to_string(),
                    parsed_line.0,
                    parsed_line.1,
                ),
                None => {
                    println!("Unhandled event {:#?}", v);
                    return;
                }
            };

            match v {
                Ok(v) => println!("{},", v),
                Err(e) => panic!(e),
            }
        }
    }
    println!("[]]");
}

fn main() -> Result<()> {
    let res =
        load_templates(Path::new("./src/event_structures")).expect("Can't find event structures.");
    let json = map_to_json(res);

    let map = create_hashmap(json);

    read_lines(map);

    Ok(())
}
