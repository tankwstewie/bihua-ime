use crossterm::event::{self, Event, KeyCode, KeyEvent};
use engine::{Stroke, StrokeDatabase};
use std::fs;
use std::io::{self, Write};

fn update(q: &[Stroke], db: &StrokeDatabase) -> Vec<String> {
    let mut out = db.query(q);
    out.sort_by_key(|c| (c.chars().next().unwrap() as u32, c.clone()));
    out
}

fn clear_and_draw(query: &[Stroke], candidates: &[String], selected: usize) {
    print!("\x1B[2J\x1B[H"); // clear screen, move cursor home
    println!("筆畫輸入法 Prototype");
    println!("Numpad: 7=一 8=丨 9=丿 4=丶/㇏ 5=乙/㇕ 6=通配符");
    println!("Keys: Enter=commit selected, 1-9=select, Esc/C=clear, q=quit");
    println!();
    let seq: String = query
        .iter()
        .map(|s| match s {
            Stroke::Heng => '7',
            Stroke::Shu => '8',
            Stroke::Pie => '9',
            Stroke::DianNa => '4',
            Stroke::Zhe => '5',
            Stroke::Wildcard => '6',
        })
        .collect();
    println!("Query: {}", seq);
    println!();
    if candidates.is_empty() {
        println!("No candidates");
    } else {
        for (i, c) in candidates.iter().take(9).enumerate() {
            let marker = if i == selected { ">" } else { " " };
            println!("{} {} {}", marker, i + 1, c);
        }
        if candidates.len() > 9 {
            println!("... and {} more", candidates.len() - 9);
        }
    }
    io::stdout().flush().unwrap();
}

fn main() {
    let bytes = fs::read("data/stroke_index.bin").expect("load stroke_index.bin");
    let db: StrokeDatabase = bincode::deserialize(&bytes).expect("deserialize db");

    let mut query: Vec<Stroke> = Vec::new();
    let mut selected: usize = 0;
    let mut candidates = update(&query, &db);
    clear_and_draw(&query, &candidates, selected);

    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read().expect("read key") {
            match code {
                KeyCode::Char('q') => break,
                KeyCode::Esc | KeyCode::Char('c') => {
                    query.clear();
                    selected = 0;
                }
                KeyCode::Char(d) if d.is_ascii_digit() => {
                    let k = d as u8 - b'0';
                    if let Some(stroke) = Stroke::from_numpad(k) {
                        query.push(stroke);
                        selected = 0;
                    } else if k > 0 && k <= 9 {
                        let idx = (k as usize) - 1;
                        if idx < candidates.len() {
                            selected = idx;
                            println!("\nCommitted: {}\n", candidates[selected]);
                            io::stdout().flush().unwrap();
                            query.clear();
                            selected = 0;
                        }
                    }
                }
                KeyCode::Enter => {
                    if !candidates.is_empty() {
                        println!("\nCommitted: {}\n", candidates[selected]);
                        io::stdout().flush().unwrap();
                        query.clear();
                        selected = 0;
                    }
                }
                KeyCode::Backspace => {
                    query.pop();
                    selected = 0;
                }
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected + 1 < candidates.len().min(9) {
                        selected += 1;
                    }
                }
                _ => {}
            }
            candidates = update(&query, &db);
            if selected >= candidates.len().min(9) {
                selected = 0;
            }
            clear_and_draw(&query, &candidates, selected);
        }
    }
}
