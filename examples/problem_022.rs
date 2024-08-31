use std::fs;

use itertools::Itertools;

fn main() {
    println!(
        "{}",
        fs::read_to_string("data/022_names.txt")
            .unwrap()
            .split(',')
            .map(|s| s.trim().trim_matches('"'))
            .sorted()
            .enumerate()
            .map(|(idx, line)| (idx, _name_score(line)))
            .map(|(idx, score)| (idx + 1) * score)
            .sum::<usize>()
    );
}

fn _name_score(line: &str) -> usize {
    line.to_lowercase()
        .chars()
        .filter(|&c| c >= 'A' && c <= 'z')
        .map(_char_score)
        .sum::<usize>()
}

fn _char_score(ch: char) -> usize {
    ch as usize
        - if ch >= 'a' {
            'a' as usize
        } else {
            'A' as usize
        }
        + 1 // normalize to 1-indexed number (i.e. 'a' should be 1)
}
