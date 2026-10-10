use engine::{classifier::classify_stroke, CharacterEntry, Stroke, StrokeDatabase};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let data_dir = Path::new("data/raw/animcjk/data/animCJK/ZhHant");
    let all_path = data_dir.join("all.json");

    if !all_path.exists() {
        eprintln!("Cannot find {}", all_path.display());
        std::process::exit(1);
    }

    let raw: HashMap<String, Value> =
        serde_json::from_str(&fs::read_to_string(&all_path).expect("read all.json"))
            .expect("parse all.json");

    let mut db = StrokeDatabase::default();
    let mut skipped = 0u32;
    let mut included = 0u32;

    for (character, value) in raw {
        let Some(medians) = value.get("medians").and_then(|m| m.as_array()) else {
            skipped += 1;
            continue;
        };

        let mut strokes = Vec::with_capacity(medians.len());
        for med in medians {
            let pts: Vec<(i32, i32)> = med
                .as_array()
                .expect("median array")
                .iter()
                .map(|p| {
                    let arr = p.as_array().expect("point array");
                    (
                        arr[0].as_i64().unwrap() as i32,
                        arr[1].as_i64().unwrap() as i32,
                    )
                })
                .collect();

            match classify_stroke(&pts) {
                Some(7) => strokes.push(Stroke::Heng),
                Some(8) => strokes.push(Stroke::Shu),
                Some(9) => strokes.push(Stroke::Pie),
                Some(4) => strokes.push(Stroke::DianNa),
                Some(5) => strokes.push(Stroke::Zhe),
                _ => {
                    skipped += 1;
                    continue;
                }
            }
        }

        if strokes.is_empty() {
            skipped += 1;
            continue;
        }

        db.insert(CharacterEntry {
            character: character.clone(),
            total_strokes: strokes.len() as u16,
            strokes,
        });
        included += 1;
    }

    let out_dir = Path::new("data");
    fs::create_dir_all(out_dir).expect("create data dir");
    let encoded = bincode::serialize(&db).expect("serialize db");
    let out_path = out_dir.join("stroke_index.bin");
    let mut file = fs::File::create(&out_path).expect("create index file");
    file.write_all(&encoded).expect("write index file");

    println!("Included: {included}, Skipped: {skipped}");
    println!("Wrote {} bytes to {}", encoded.len(), out_path.display());
}
