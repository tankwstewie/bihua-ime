# bihua-ime

Windows 筆畫輸入法 for Traditional Chinese, driven by the numeric keypad.

## Phase 1: Cross-platform prototype

The current repo contains the matching engine and a terminal-based prototype.

```bash
# Build everything
cargo build --workspace

# Download AnimCJK data and build the stroke index
cargo run --bin build_index

# Verify the engine against known queries
cargo run --bin verify_queries

# Run the interactive terminal prototype (requires a terminal with numpad)
cargo run --bin prototype
```

## Numpad mapping

| Key | Stroke | Name |
|-----|--------|------|
| 7   | 一     | 橫   |
| 8   | 丨     | 豎   |
| 9   | 丿     | 撇   |
| 4   | 丶/㇏  | 點/捺 |
| 5   | 乙/㇕  | 折   |
| 6   | *      | 通配符 |

## Data source

Stroke geometry from [AnimCJK](https://github.com/parsimonhi/animCJK), repackaged by [hanzi-writer-data-acjk](https://github.com/akkrat/hanzi-writer-data-acjk). Stroke classification is derived from the geometric approach used by HanziLookupJS.

## Phase 2 (planned)

Wrap the Rust engine in a Windows TSF (Text Services Framework) DLL so it appears as a real input method in the language bar.
