use crate::lib_2017::knot_hash;
use itertools::Itertools;

use crate::lib_aoc::unionfind::NamedUnionFind;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let key_string = input.trim(); // Test input: "flqrgnkx";
    let hashes: Vec<_> = (0..128)
        .map(|row| knot_hash(format!("{key_string}-{row}").as_bytes()))
        .collect();

    // Part 1
    let bit_count = hashes.iter().flatten().map(|b| b.count_ones()).sum::<u32>();

    // Part 2
    let mut uf = NamedUnionFind::new();
    for (row, hash) in hashes.iter().enumerate() {
        for (index, byte) in hash.iter().enumerate() {
            for digit in 0..8 {
                // NB: need to move from left (most-significant) to right, or else connecting to neighbours goes wrong
                if byte & (0b10000000 >> digit) != 0 {
                    let col = 8 * index + digit;
                    uf.add((col, row));
                    // Only nodes to the top and left have been evaluated;
                    // bottom and right neighbours will connect when they are evaluated
                    // Use the fact that `connect` is a no-op if either label is invalid
                    if col > 0 {
                        uf.connect((col - 1, row), (col, row));
                    }
                    if row > 0 {
                        uf.connect((col, row - 1), (col, row));
                    }
                }
            }
        }
    }
    let group_count = uf
        .get_labels()
        .map(|label| uf.get_id(*label))
        .sorted()
        .unique()
        .count();

    (bit_count.to_string(), group_count.to_string())
}
