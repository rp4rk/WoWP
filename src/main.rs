mod matcher;
mod parser;

use linereader::LineReader;
use matcher::loader::{create_hashmap, load_templates, map_to_json};
use parser::types::{LogCell, LogEventDateTime};
use serde_json::Value;
use snafu::{ResultExt, Snafu};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::str::{from_utf8, Utf8Error};
use std::{fs::File, io};

#[derive(Debug, Snafu)]
enum WowpError {
    #[snafu(display("Failed to load event templates from {}: {}", path.display(), source))]
    TemplateLoadFailed { source: io::Error, path: PathBuf },
    #[snafu(display("Failed to find an event map"))]
    EventMapNotFound,
    #[snafu(display("Failed to serialize event"))]
    SerializationFailed { source: serde_json::Error },
    #[snafu(display("Failed to read log line"))]
    LogLineReadFailed { source: io::Error },
    #[snafu(display("Failed to convert log line into a string slice"))]
    LogLineConversionFailed { source: Utf8Error },
    #[snafu(display("Failed to link event to structure"))]
    EventStructureLinkFailed,
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

fn whitelisted_events() -> HashSet<String> {
    let mut events = HashSet::new();

    events.insert("COMBAT_LOG_VERSION".to_string());
    events.insert("ZONE_CHANGE".to_string());

    events
}

fn parse_lines(map: HashMap<String, Value>, line_reader_config: LineReaderConfig) -> Result<()> {
    let file = File::open("WoWCombatLog.txt").expect("open");
    let mut reader = LineReader::new(file);

    // Setup parsing configuration
    let whitelisted_events = whitelisted_events();
    let mut parse_enabled = line_reader_config.parse_trash;

    while let Some(line) = reader.next_line() {
        let log_line = line.context(LogLineReadFailed)?;
        let string = from_utf8(log_line).context(LogLineConversionFailed)?;
        let parsed_line = parser::parse_log_line(string);

        if let LogCell::Str(event_type) = parsed_line.1[0] {
            if event_type == "ENCOUNTER_START" && line_reader_config.parse_trash == false {
                parse_enabled = true;
            }

            // Ignore parsing if it's disabled
            if parse_enabled || whitelisted_events.contains(event_type) {
                let event_map_result = map
                    .get(event_type)
                    .ok_or_else(|| WowpError::EventStructureLinkFailed)?;

                let event_json = create_json_representation(
                    event_map_result,
                    event_type.to_string(),
                    parsed_line.0,
                    parsed_line.1,
                )?;
            }

            if event_type == "ENCOUNTER_END" && line_reader_config.parse_trash == false {
                parse_enabled = false;
            }
        }
    }

    Ok(())
}

struct LineReaderConfig {
    parse_trash: bool,
}

impl Default for LineReaderConfig {
    fn default() -> LineReaderConfig {
        LineReaderConfig { parse_trash: false }
    }
}

fn main() -> Result<()> {
    // Loads event templates to be used to match against events
    let path = Path::new("./src/event_structures");
    let event_template_paths = load_templates(path).context(TemplateLoadFailed { path })?;

    // Turns the provided paths into JSON maps
    let event_json_maps = map_to_json(event_template_paths);
    let event_maps = create_hashmap(event_json_maps);

    // Determine configuration for reading lines
    let line_reader_config = LineReaderConfig {
        // parse_trash: true,
        ..Default::default()
    };

    // Hand off to line parser
    parse_lines(event_maps, line_reader_config);

    Ok(())
}
