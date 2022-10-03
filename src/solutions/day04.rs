use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut set = HashSet::new();
    let mut part1 = 0;
    for line in input.lines() {
        set.clear();
        if line
            .split_ascii_whitespace()
            .all(|word| set.insert(word.to_string()))
        {
            part1 += 1;
        }
    }

    let mut part2 = 0;
    for line in input.lines() {
        set.clear();
        if line
            .split_ascii_whitespace()
            .all(|word| set.insert(word.chars().sorted().collect()))
        {
            part2 += 1;
        }
    }

    (part1.to_string(), part2.to_string())
}
