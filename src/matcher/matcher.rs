use serde_json::{self, Value};
use std::collections::HashMap;

fn recursive_match<'a, T>(
    accumulator: &'a mut HashMap<&'a String, &'a T>,
    template: &'a Value,
    event_values: &'a Vec<T>,
) -> &'a HashMap<&'a String, &'a T> {
    let template_obj = template.as_object().unwrap();

    for (key, value) in template_obj {
        if let Some(v) = value.as_array() {
            if let Some(nested_template) = v.get(1).unwrap().as_object() {
                let idx = v.get(0).unwrap().as_i64().unwrap() as usize;
            }
        }
        // Index detected, append the value
        if let Some(v) = value.as_u64() {
            accumulator.insert(key, &(event_values[v as usize]));
        }
    }

    return accumulator;
}

#[test]
fn test_recursive_match() {
    let recursive_json = r#"
    {
      "name": 0,
      "covenant": [
        1,
        {
          "soulbindId": 0,
          "covenantId": 1,
          "soulbindPath": 3,
          "conduits": 4
        }
      ],
      "gear": [
        2,
        {
          "itemId": 0
        }
      ]
    }"#;

    let v: Value = serde_json::from_str(recursive_json).unwrap();
    let values = vec!["test", "test2"];
    let mut hmap = HashMap::new();

    let v2 = recursive_match(&mut hmap, &v, &values);

    println!("{:#?}", v2);
}
