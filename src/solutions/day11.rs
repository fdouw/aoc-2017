pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut coords = Hex::new();
    let origin = Hex::new();

    let max_dist = input
        .trim()
        .split(',')
        .map(|d| {
            coords.step(d);
            coords.dist(&origin)
        })
        .max()
        .unwrap();
    let part1 = coords.dist(&origin);

    (part1.to_string(), max_dist.to_string())
}

struct Hex {
    q: i64,
    r: i64,
    s: i64,
}

impl Hex {
    fn new() -> Self {
        Self { q: 0, r: 0, s: 0 }
    }
    fn step(&mut self, dir: &str) {
        match dir {
            "n" => {
                self.r -= 1;
                self.s += 1;
            }
            "ne" => {
                self.q += 1;
                self.r -= 1;
            }
            "se" => {
                self.q += 1;
                self.s -= 1;
            }
            "s" => {
                self.r += 1;
                self.s -= 1;
            }
            "sw" => {
                self.q -= 1;
                self.r += 1;
            }
            "nw" => {
                self.q -= 1;
                self.s += 1;
            }
            _ => {
                panic!("Invalid direction: '{dir}'");
            }
        }
    }
    fn dist(&self, other: &Hex) -> u64 {
        // See https://www.redblobgames.com/grids/hexagons/#distances
        (self.q.abs_diff(other.q) + self.r.abs_diff(other.r) + self.s.abs_diff(other.s)) / 2
    }
}
