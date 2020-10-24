mod matcher;
mod parser;

use linereader::LineReader;
use matcher::loader::{create_hashmap, load_templates, map_to_json};
use parser::types::{LogCell, LogEventDateTime};
use serde_json::Value;
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::from_utf8;
use std::{fs::File, io};

#[derive(Debug, Snafu)]
enum WowpError {
    #[snafu(display("Failed to load event templates from {}: {}", path.display(), source))]
    TemplateLoadFailed { source: io::Error, path: PathBuf },
    #[snafu(display("Failed to find an event map"))]
    EventMapNotFound,
    #[snafu(display("Failed to serialize event"))]
    SerializationFailed { source: serde_json::Error },
}

type Result<T, E = WowpError> = std::result::Result<T, E>;

fn create_json_representation(
    event_map: &Value,
    event_type: String,
    time: LogEventDateTime,
    event: Vec<LogCell>,
) -> Result<String> {
    // Unwrap the event map
    let event_map_obj = event_map
        .as_object()
        .ok_or_else(|| WowpError::EventMapNotFound)?;

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

    // Inserts the event location by default at index 0
    // This allows us to omit it in templates by default
    let key = &"eventType".to_string();
    let value = &LogCell::Str(event_type.as_str());
    hmap.insert(key, value);

    let json_string = serde_json::to_string_pretty(&hmap).context(SerializationFailed)?;

    Ok(json_string)
}

fn parse_lines(map: HashMap<String, Value>) {
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
                    eprintln!("Unhandled event {:#?}", v);
                    return;
                }
            };

            let newV = match v {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{:#?}", e);
                    panic!("fuck")
                }
            };

            println!("{}", newV);
        }
    }
    println!("[]]");
}

fn main() -> Result<()> {
    // Loads event templates to be used to match against events
    let path = Path::new("./src/event_structures");
    let event_template_paths = load_templates(path).context(TemplateLoadFailed { path })?;

    // Turns the provided paths into JSON maps
    let event_json_maps = map_to_json(event_template_paths);
    let event_maps = create_hashmap(event_json_maps);

    // Start processing lines
    parse_lines(event_maps);

    Ok(())
}
