use engine::{Stroke, StrokeDatabase};
use std::fs;

fn strokes(seq: &str) -> Vec<Stroke> {
    seq.chars()
        .filter_map(|c| {
            c.to_digit(10)
                .map(|d| Stroke::from_numpad(d as u8).unwrap())
        })
        .collect()
}

fn main() {
    let bytes = fs::read("data/stroke_index.bin").expect("load stroke_index.bin");
    let db: StrokeDatabase = bincode::deserialize(&bytes).expect("deserialize db");

    let cases: Vec<(&str, &str, bool)> = vec![
        ("7", "一", true),      // 一 = 橫
        ("8", "丨", true),      // 丨 = 豎
        ("9", "丿", true),      // 丿 = 撇
        ("4", "丶", true),      // 丶 = 點
        ("794", "大", true),    // 大 = 7,9,4 (橫, 撇, 捺)
        ("7894", "木", true),   // 木 = 橫,豎,撇,捺
        ("5894", "水", true),   // 水 = 豎鉤? Actually 水 is 5,8,9,4
        ("495497", "字", true), // 字 full sequence
        ("4954", "字", true),   // 字 prefix
        ("6954", "字", true),   // 字 with wildcard at position 1
        ("7956", "字", false),  // unlikely wildcard combo
    ];

    let mut pass = 0u32;
    let mut fail = 0u32;
    for (query, expected, should_find) in cases {
        let q = strokes(query);
        let results = db.query(&q);
        let found = results.iter().any(|r| r == expected);
        let ok = found == should_find;
        if ok {
            pass += 1;
            println!(
                "OK   {} -> expect {} found={} ({} results)",
                query,
                expected,
                found,
                results.len()
            );
        } else {
            fail += 1;
            println!(
                "FAIL {} -> expect {} (find={}), found={} ({} results)",
                query,
                expected,
                should_find,
                found,
                results.len()
            );
        }
    }

    println!("\nPassed: {}, Failed: {}", pass, fail);
    if fail > 0 {
        std::process::exit(1);
    }
}
