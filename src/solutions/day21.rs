use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    #[allow(unused_variables)]
    let test_input = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";

    // Starting grid is given in problem statement:
    // .#.
    // ..#
    // ###
    let start_grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let rules = Rules::new(&input);

    let mut grid = start_grid;
    for _ in 0..5 {
        grid = transform(grid, &rules);
    }
    let part1 = grid.iter().flatten().filter(|b| **b).count();
    for _ in 0..13 {
        grid = transform(grid, &rules);
    }
    let part2 = grid.iter().flatten().filter(|b| **b).count();

    (part1.to_string(), part2.to_string())
}

#[allow(non_snake_case)]
fn transform(grid: Vec<Vec<bool>>, rules: &Rules) -> Vec<Vec<bool>> {
    // grid is a size * size grid
    let size = grid.len();

    // Size of the pattern were going to match and number of blocks on each side in the input
    let pat_size;
    if size % 2 == 0 {
        pat_size = 2;
    } else if size % 3 == 0 {
        pat_size = 3;
    } else {
        panic!("ERROR || size {size} not divisible by 2 or 3.");
    }
    let blocks_side = size / pat_size;

    // Note on coordinates:
    // X,Y index cells in the original grid (0..size)
    // x,y index cells in the original blocks (0..pat_size)
    // U,V index cells in the resulting grid (0..new_size)
    // u,v index cells in the resulting blocks (0..pat_size+1)
    // i,j are the coordinates of the blocks (0..blocks_side)

    // temporary vector to store the intermediate blocks
    let mut block_vec = Vec::with_capacity(pat_size * pat_size);
    (0..(pat_size * pat_size)).for_each(|_| block_vec.push(false));

    // new vector to store the result of the transformation
    let new_size = size + blocks_side;
    let mut new_grid = Vec::with_capacity(new_size);
    for V in 0..new_size {
        new_grid.push(Vec::with_capacity(new_size));
        for _U in 0..new_size {
            new_grid[V].push(false);
        }
    }

    for j in 0..blocks_side {
        for i in 0..blocks_side {
            // Extract the block at (i, j)
            for y in 0..pat_size {
                for x in 0..pat_size {
                    let idx = y * pat_size + x;
                    let X = i * pat_size + x;
                    let Y = j * pat_size + y;
                    block_vec[idx] = grid[Y][X];
                }
            }
            // Transform the block and insert the result into the new grid
            let transformed: &Vec<bool> = rules.transform(&block_vec);
            for v in 0..pat_size + 1 {
                for u in 0..pat_size + 1 {
                    new_grid[j * (pat_size + 1) + v][i * (pat_size + 1) + u] =
                        transformed[v * (pat_size + 1) + u];
                }
            }
        }
    }

    new_grid
}

struct Rules {
    rules: HashMap<Vec<bool>, Vec<bool>>,
}

impl Rules {
    fn new(data: &str) -> Self {
        let mut rules = HashMap::new();
        for entry in data.lines() {
            let (pattern, result) = entry
                .split(" => ")
                .map(|s| {
                    s.replace("/", "")
                        .chars()
                        .map(|c| c == '#')
                        .collect::<Vec<bool>>()
                })
                .collect_tuple()
                .unwrap();
            let symmetries = PatternSymmetries::new(pattern);
            for pat in symmetries {
                rules.insert(pat, result.clone());
            }
        }
        Rules { rules }
    }
    fn transform(&self, pattern: &Vec<bool>) -> &Vec<bool> {
        match self.rules.get(pattern) {
            Some(r) => r,
            None => panic!("No rule for pattern: {:?}", pattern),
        }
    }
}

struct PatternSymmetries {
    pattern: Vec<bool>,
    size: usize,
    counter: u32,
}

impl PatternSymmetries {
    fn new(pattern: Vec<bool>) -> Self {
        let size = f64::sqrt(pattern.len() as f64) as usize;
        PatternSymmetries {
            pattern,
            size,
            counter: 0,
        }
    }
    fn rotate(&mut self) {
        let old_pat = self.pattern.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                let u = self.size - 1 - y;
                let v = x;
                self.pattern[y * self.size + x] = old_pat[v * self.size + u];
            }
        }
    }
    fn flip(&mut self) {
        // flip across horizontal axis
        let old_pat = self.pattern.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                self.pattern[y * self.size + x] = old_pat[(self.size - 1 - y) * self.size + x];
            }
        }
    }
}

impl Iterator for PatternSymmetries {
    type Item = Vec<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            self.counter += 1;
            Some(self.pattern.clone())
        } else if self.counter == 4 {
            self.counter += 1;
            self.flip();
            Some(self.pattern.clone())
        } else if self.counter < 8 {
            self.counter += 1;
            self.rotate();
            Some(self.pattern.clone())
        } else {
            None
        }
    }
}

#[test]
fn test_symmetries_1100() {
    let pattern = vec![true, true, false, false];
    let symmetries = PatternSymmetries::new(pattern.clone());
    let results: HashSet<Vec<bool>> = symmetries.collect();

    assert!(
        results.contains(&pattern),
        "Find original pattern among results"
    );
    assert!(
        results.contains(&vec![true, false, true, false]),
        "Find 1x CCW rotation among results"
    );
    assert!(
        results.contains(&vec![false, false, true, true]),
        "Find 2x CCW rotation among results"
    );
    assert!(
        results.contains(&vec![false, true, false, true]),
        "Find 3x CCW rotation among results"
    );
    assert_eq!(
        4,
        results.len(),
        "Check that there are exactly 4 symmetries"
    );
}

#[test]
fn test_symmetries_1001() {
    let pattern = vec![true, false, false, true];
    let symmetries = PatternSymmetries::new(pattern.clone());
    let results: HashSet<Vec<bool>> = symmetries.collect();

    assert!(
        results.contains(&pattern),
        "Find original pattern among results"
    );
    assert!(
        results.contains(&vec![false, true, true, false]),
        "Find rotation among results"
    );
    assert_eq!(
        2,
        results.len(),
        "Check that there are exactly 2 symmetries"
    );
}

#[test]
fn test_symmetries_010001111() {
    let pattern = vec![false, true, false, false, false, true, true, true, true];
    let symmetries = PatternSymmetries::new(pattern.clone());
    let results: HashSet<Vec<bool>> = symmetries.collect();

    assert!(
        results.contains(&pattern),
        "Find original pattern among results"
    );
    assert!(
        results.contains(&vec![
            false, true, true, true, false, true, false, false, true
        ]),
        "Find 1x CCW rotation among results"
    );
    assert!(
        results.contains(&vec![
            true, true, true, true, false, false, false, true, false
        ]),
        "Find 2x CCW rotation among results"
    );
    assert!(
        results.contains(&vec![
            true, false, false, true, false, true, true, true, false
        ]),
        "Find 3x CCW rotation among results"
    );
    assert!(
        results.contains(&vec![
            true, true, false, true, false, true, true, false, false
        ]),
        "Find 3x CCW, flip, among results"
    );
    assert!(
        results.contains(&vec![
            false, true, false, true, false, false, true, true, true
        ]),
        "Find 3x CCW, flip, 1x CCW among results"
    );
    assert!(
        results.contains(&vec![
            false, false, true, true, false, true, false, true, true
        ]),
        "Find 3x CCW, flip, 2x CCW among results"
    );
    assert!(
        results.contains(&vec![
            true, true, true, false, false, true, false, true, false
        ]),
        "Find 3x CCW, flip, 3x CCW among results"
    );
    assert_eq!(
        8,
        results.len(),
        "Check that there are exactly 8 symmetries"
    )
}

#[test]
fn test_symmetries_010001111_iter() {
    let pattern = vec![false, true, false, false, false, true, true, true, true];
    let mut symmetries = PatternSymmetries::new(pattern.clone());

    assert!(
        symmetries.next().unwrap() == pattern,
        "Find original pattern among results"
    );
    let next = symmetries.next().unwrap();
    assert!(
        next == vec![false, true, true, true, false, true, false, false, true],
        "Find 1x CCW rotation among results: {:?}",
        next
    );
    let next = symmetries.next().unwrap();
    assert!(
        next == vec![true, true, true, true, false, false, false, true, false],
        "Find 2x CCW rotation among results: {:?}",
        next
    );
    let next = symmetries.next().unwrap();
    assert!(
        next == vec![true, false, false, true, false, true, true, true, false],
        "Find 3x CCW rotation among results: {:?}",
        next
    );
    let next = symmetries.next().unwrap();
    assert!(
        next == vec![true, true, false, true, false, true, true, false, false],
        "Find 3x CCW, flip, among results: {:?}",
        next
    );
    let next = symmetries.next().unwrap();
    assert!(
        next == vec![false, true, false, true, false, false, true, true, true],
        "Find 3x CCW, flip, 1x CCW among results: {:?}",
        next
    );
    let next = symmetries.next().unwrap();
    assert!(
        next == vec![false, false, true, true, false, true, false, true, true],
        "Find 3x CCW, flip, 2x CCW among results: {:?}",
        next
    );
    let next = symmetries.next().unwrap();
    assert!(
        next == vec![true, true, true, false, false, true, false, true, false],
        "Find 3x CCW, flip, 3x CCW among results: {:?}",
        next
    );
    assert!(
        symmetries.next().is_none(),
        "Check that there are exactly 8 symmetries"
    );
}
