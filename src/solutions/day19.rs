use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // Scan top row to find '|'
    // keep track of position and direction
    // '|', or '-': follow along
    //   -> they do not point the right way in case of crossing paths
    // <letter>: register letter, then follow along
    // '+': find path in different direction
    //   -> remember where we came from
    //   -> beware of parallel lines

    // Read the map
    let mut grid = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                '|' | '-' => Cell::Path,
                ' ' => Cell::Empty,
                '+' => Cell::Corner,
                'A'..='Z' => Cell::Letter(c),
                _ => panic!("ERROR | Invalid character in input: {c}"),
            })
            .collect::<Vec<_>>();
        grid.push(row);
    }

    // Find the starting point
    let mut current_direction = &Direction::South;
    let mut x = 0;
    let mut y = 0;
    for (i, cell) in grid[0].iter().enumerate() {
        if *cell == Cell::Path {
            x = i;
        }
    }

    // Follow the path
    let mut text = String::new();
    let deltas: HashMap<Direction, (i64, i64)> = [
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]
    .into_iter()
    .collect();
    let mut steps = 0;
    'walk_path: loop {
        steps += 1;
        match grid[y][x] {
            Cell::Path => match current_direction {
                Direction::North => y -= 1,
                Direction::East => x += 1,
                Direction::South => y += 1,
                Direction::West => x -= 1,
            },
            Cell::Letter(c) => {
                text.push(c);
                match current_direction {
                    Direction::North => y -= 1,
                    Direction::East => x += 1,
                    Direction::South => y += 1,
                    Direction::West => x -= 1,
                }
            }
            Cell::Corner => {
                // Assume there's only one way to go
                for (dir, (dx, dy)) in deltas.iter() {
                    // Don't look in the direction we came from, or outside the grid
                    if !opposite_dirs(dir, current_direction) {
                        if let Some((u, v)) = valid_coords(x, *dx, y, *dy, &grid) {
                            current_direction = dir;
                            (x, y) = (u, v);
                            continue 'walk_path;
                        }
                    }
                }
                // No more options, assume end of the road
                break;
            }
            Cell::Empty => {
                // Empty spot, assume end of the road
                steps -= 1; // Do not count empty cell
                break;
            }
        }
    }

    (text, steps.to_string())
}

fn valid_coords(
    x: usize,
    dx: i64,
    y: usize,
    dy: i64,
    grid: &Vec<Vec<Cell>>,
) -> Option<(usize, usize)> {
    let width = grid[0].len();
    let height = grid.len();
    let safe_coords = match (dx, dy) {
        (0, -1) => y > 0,
        (1, 0) => x < width - 1,
        (0, 1) => y < height - 1,
        (-1, 0) => x > 0,
        _ => panic!("ERROR || Invalid deltas for moving"),
    };
    if safe_coords {
        let (u, v) = ((x as i64 + dx) as usize, (y as i64 + dy) as usize);
        if grid[v][u] != Cell::Empty {
            Some((u, v))
        } else {
            None
        }
    } else {
        None
    }
}

fn opposite_dirs(a: &Direction, b: &Direction) -> bool {
    match a {
        Direction::North => b == &Direction::South,
        Direction::East => b == &Direction::West,
        Direction::South => b == &Direction::North,
        Direction::West => b == &Direction::East,
    }
}

#[derive(Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq)]
enum Cell {
    Empty,
    Path,
    Corner,
    Letter(char),
}
