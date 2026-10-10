pub mod classifier;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 5 canonical stroke types used by the iPhone 筆畫 input method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Stroke {
    /// 橫 / horizontal (numpad 7)
    Heng = 7,
    /// 豎 / vertical (numpad 8)
    Shu = 8,
    /// 撇 / left-falling (numpad 9)
    Pie = 9,
    /// 點/捺 / dot/right-falling (numpad 4)
    DianNa = 4,
    /// 折 / hook/turn (numpad 5)
    Zhe = 5,
    /// 通配符 / wildcard (numpad 6)
    Wildcard = 6,
}

impl Stroke {
    pub fn from_numpad(k: u8) -> Option<Stroke> {
        match k {
            7 => Some(Stroke::Heng),
            8 => Some(Stroke::Shu),
            9 => Some(Stroke::Pie),
            4 => Some(Stroke::DianNa),
            5 => Some(Stroke::Zhe),
            6 => Some(Stroke::Wildcard),
            _ => None,
        }
    }
}

/// A single character entry in the index.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterEntry {
    pub character: String,
    pub strokes: Vec<Stroke>,
    pub total_strokes: u16,
}

/// Trie node for prefix + wildcard queries.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TrieNode {
    /// Characters that end at exactly this stroke sequence.
    pub results: Vec<String>,
    /// Children keyed by Stroke discriminant (7,8,9,4,5).
    pub children: BTreeMap<u8, TrieNode>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StrokeDatabase {
    pub root: TrieNode,
}

impl StrokeDatabase {
    pub fn insert(&mut self, entry: CharacterEntry) {
        let mut node = &mut self.root;
        for stroke in &entry.strokes {
            node = node.children.entry(*stroke as u8).or_default();
        }
        node.results.push(entry.character);
    }

    /// Query by a sequence of numpad digits. Returns matching characters.
    /// Supports prefix matching and wildcard `6`.
    pub fn query(&self, query: &[Stroke]) -> Vec<String> {
        let mut out = Vec::new();
        self.collect(&self.root, query, 0, &mut out);
        out
    }

    fn collect(&self, node: &TrieNode, query: &[Stroke], depth: usize, out: &mut Vec<String>) {
        // If we've consumed the whole query, all results under this node are valid
        // because we are doing prefix matching.
        if depth == query.len() {
            self.gather_results(node, out);
            return;
        }

        let stroke = query[depth];
        if stroke == Stroke::Wildcard {
            for child in node.children.values() {
                self.collect(child, query, depth + 1, out);
            }
        } else if let Some(child) = node.children.get(&(stroke as u8)) {
            self.collect(child, query, depth + 1, out);
        }
    }

    fn gather_results(&self, node: &TrieNode, out: &mut Vec<String>) {
        out.extend(node.results.iter().cloned());
        for child in node.children.values() {
            self.gather_results(child, out);
        }
    }
}
