use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

type PathAssoc = Vec<(OsString, PathBuf)>;
type PathJsonAssoc = Vec<(OsString, Value)>;

pub fn load_templates(dir: &Path) -> Result<PathAssoc, io::Error> {
    let paths: PathAssoc = fs::read_dir(dir)?
        .flat_map(|f| f.ok())
        .map(|f| (f.file_name(), f.path()))
        .collect();

    Ok(paths)
}

pub fn map_to_json(paths: PathAssoc) -> PathJsonAssoc {
    paths
        .into_iter()
        .map(|s| (s.0, fs::read_to_string(s.1).unwrap()))
        .map(|r| (r.0, from_str(r.1.as_ref()).unwrap()))
        .collect()
}

fn split_file_name(input: OsString) -> String {
    let str = input.into_string().unwrap();
    let res: Vec<&str> = str.split('.').collect();

    String::from(res[0])
}

pub fn create_hashmap(input: PathJsonAssoc) -> HashMap<String, Value> {
    let mut json_hash = HashMap::new();

    input.into_iter().for_each(|f| {
        json_hash.insert(split_file_name(f.0), f.1);
    });

    return json_hash;
}
