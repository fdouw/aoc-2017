use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // For testing
    // let input = "..#\n#..\n...";

    // assume a square input
    let size = input.trim().lines().count() as i32;

    // Part 1
    let mut grid = HashMap::new();
    for (row, line) in input.trim().lines().enumerate() {
        for (col, char) in line.char_indices() {
            grid.insert((col as i32, row as i32), char == '#');
        }
    }

    let mut pos = (size / 2, size / 2);
    let mut dir = Direction::North;
    let mut infections = 0;
    for _ in 0..10_000 {
        if *grid.get(&pos).unwrap_or(&false) {
            // Currently infected
            dir.turn(Turn::Right);
            grid.insert(pos, false);
        } else {
            // Currently clean
            dir.turn(Turn::Left);
            grid.insert(pos, true);
            infections += 1;
        }
        // Move
        pos = dir.step(pos);
    }

    // Part 2
    let mut grid = HashMap::new();
    for (row, line) in input.trim().lines().enumerate() {
        for (col, char) in line.char_indices() {
            if char == '#' {
                grid.insert((col as i32, row as i32), State::Infected);
            }
        }
    }

    // Maybe complex numbers instead?
    let mut pos = (size / 2, size / 2);
    let mut dir = Direction::North;
    let mut infections2 = 0;
    for _ in 0..10_000_000 {
        let node = grid.entry(pos).or_insert(State::Clean);
        match node {
            State::Clean => {
                dir.turn(Turn::Left);
                *node = State::Weakened;
            }
            State::Weakened => {
                infections2 += 1;
                *node = State::Infected;
            }
            State::Infected => {
                dir.turn(Turn::Right);
                *node = State::Flagged;
            }
            State::Flagged => {
                dir.turn(Turn::Reflect);
                *node = State::Clean;
            }
        }
        pos = dir.step(pos);
    }

    (infections.to_string(), infections2.to_string())
}

enum Direction {
    North,
    East,
    South,
    West,
}

enum Turn {
    Left,
    Right,
    Reflect,
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        *self = match turn {
            Turn::Left => match self {
                Self::North => Self::West,
                Self::East => Self::North,
                Self::South => Self::East,
                Self::West => Self::South,
            },
            Turn::Right => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
            },
            Turn::Reflect => match self {
                Self::North => Self::South,
                Self::East => Self::West,
                Self::South => Self::North,
                Self::West => Self::East,
            },
        };
    }
    fn step(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Self::North => (x, y - 1),
            Self::East => (x + 1, y),
            Self::South => (x, y + 1),
            Self::West => (x - 1, y),
        }
    }
}

enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}
