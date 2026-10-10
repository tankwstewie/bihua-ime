use engine::classifier::classify_stroke;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() {
    let raw: HashMap<String, Value> = serde_json::from_str(
        &fs::read_to_string("data/raw/animcjk/data/animCJK/ZhHant/all.json").unwrap(),
    )
    .unwrap();

    let targets = vec![
        "一", "丨", "丿", "丶", "亅", "中", "大", "木", "水", "字", "國", "九", "力",
    ];
    for c in targets {
        let Some(medians) = raw[c].get("medians").and_then(|m| m.as_array()) else {
            println!("{}: no medians", c);
            continue;
        };
        let seq: String = medians
            .iter()
            .map(|m| {
                let pts: Vec<(i32, i32)> = m
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|p| {
                        let a = p.as_array().unwrap();
                        (a[0].as_i64().unwrap() as i32, a[1].as_i64().unwrap() as i32)
                    })
                    .collect();
                match classify_stroke(&pts) {
                    Some(7) => '7',
                    Some(8) => '8',
                    Some(9) => '9',
                    Some(4) => '4',
                    Some(5) => '5',
                    _ => '?',
                }
            })
            .collect();
        println!("{}: {}", c, seq);
    }
}
