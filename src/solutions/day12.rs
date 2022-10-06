use crate::lib_aoc::unionfind::UnionFind;
use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let size = input.lines().count();
    let mut uf = UnionFind::new(size);

    for (id, line) in input.lines().enumerate() {
        // Use the fact each id comes on its own line, so we can ignore the first part and use the line number instead
        let neighbours = line
            .split(" <-> ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<usize>().unwrap());
        for nb in neighbours {
            uf.connect(id, nb);
        }
    }
    let part1 = uf.count(0);
    let part2 = (0..size).map(|n| uf.get_id(n)).sorted().unique().count();

    (part1.to_string(), part2.to_string())
}
