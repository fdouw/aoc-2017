use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let target: u64 = input.parse().unwrap();

    // Part 1
    // In Manhattan distance, we should walk to one of the axes and then to the centre.
    // The walk to the centre is half the side of the enclosed square.
    let mut enclosed_side: u64 = unsafe { f64::sqrt(target as f64).to_int_unchecked() };
    if enclosed_side % 2 == 0 {
        enclosed_side -= 1;
    }

    let target_side = enclosed_side + 2;
    let target_square = target_side * target_side;

    let mut step = u64::max_value();
    let mut axis = target_square - (target_side - 1) / 2;
    for _ in 0..4 {
        step = step.min(target.abs_diff(axis));
        axis -= target_side - 1;
    }

    let dist = step + (enclosed_side + 1) / 2;

    // Part 2
    // Quick look on Reddit says generating the spiral is the best way
    let mut squares = SquareMap::new();
    let mut corner = 1;
    let mut x = 1;
    let mut y = 0;
    let mut part2;

    'search: loop {
        while y < corner {
            part2 = squares.compute(x, y);
            if part2 > target {
                break 'search;
            }
            y += 1;
        }
        while x > -corner {
            part2 = squares.compute(x, y);
            if part2 > target {
                break 'search;
            }
            x -= 1;
        }
        while y > -corner {
            part2 = squares.compute(x, y);
            if part2 > target {
                break 'search;
            }
            y -= 1;
        }
        // Increase side length here, so we move to the next ring automatically
        corner += 1;
        while x < corner {
            part2 = squares.compute(x, y);
            if part2 > target {
                break 'search;
            }
            x += 1;
        }
    }
    (dist.to_string(), part2.to_string())
}

struct SquareMap {
    map: HashMap<(i64, i64), u64>,
}

impl SquareMap {
    fn new() -> SquareMap {
        SquareMap {
            map: HashMap::from([((0, 0), 1)]),
        }
    }

    fn compute(&mut self, x: i64, y: i64) -> u64 {
        // Computes the value at (x,y), stores it, and returns it.
        // The sum includes the value at (x,y) itself, but this should still be zero when we compute it
        let mut sum = 0;
        for dx in 0..3 {
            for dy in 0..3 {
                sum += self.map.get(&(x + dx - 1, y + dy - 1)).unwrap_or(&0);
            }
        }
        self.map.insert((x, y), sum);
        sum
    }
}
