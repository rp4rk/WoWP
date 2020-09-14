mod matcher;
mod parser;

use linereader::LineReader;
use matcher::loader::{create_hashmap, load_templates, map_to_json};
use serde_json::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::str::from_utf8;

fn read_lines(map: HashMap<String, Value>) {
    let file = File::open("WoWCombatLog.txt").expect("open");

    let mut reader = LineReader::new(file);

    while let Some(line) = reader.next_line() {
        let line = line.expect("Read error");
        let string = from_utf8(line).expect("parse error");
        let test = parser::parse_log_line(string);

        // V BROKEN V
        // println!("{:?}", test.1[0]);
        // let key = test.1[0]
        // if map.contains_key(test.1[0]) {
        //     println!("Match!")
        // }
    }
}

fn main() -> Result<()> {
    let res = load_templates(Path::new("./src/event_structures")).unwrap();
    let json = map_to_json(res);

    let map = create_hashmap(json);

    read_lines(map);

    Ok(())
}
