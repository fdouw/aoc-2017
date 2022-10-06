use itertools::Itertools;
use regex::Regex;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let re = Regex::new("^(?P<t>.)(?P<a>[^/]+)(/(?P<b>.+))?$").unwrap();
    let mut dancers = "abcdefghijklmnop".chars().collect::<Vec<_>>();

    for dance_move in input.trim().split(",") {
        let step = re.captures(dance_move).unwrap();
        match step.name("t").unwrap().as_str() {
            "s" => {
                // Spin the line of programs
                let amount = step.name("a").unwrap().as_str().parse::<usize>().unwrap();
                dancers.rotate_right(amount);
            }
            "x" => {
                // Exchange 2 indices
                let a = step.name("a").unwrap().as_str().parse::<usize>().unwrap();
                let b = step.name("b").unwrap().as_str().parse::<usize>().unwrap();
                let tmp = dancers[a];
                dancers[a] = dancers[b];
                dancers[b] = tmp;
            }
            "p" => {
                // Partner 2 programs (ie, swap names)
                let a = step.name("a").unwrap().as_str().chars().nth(0).unwrap();
                let b = step.name("b").unwrap().as_str().chars().nth(0).unwrap();
                let mut idx_a = 0;
                let mut idx_b = 0;
                for (i, n) in dancers.iter().enumerate() {
                    if *n == a {
                        idx_a = i;
                    }
                    if *n == b {
                        idx_b = i;
                    }
                }
                dancers[idx_a] = b;
                dancers[idx_b] = a;
            }
            x => panic!("Invalid instruction: {x}"),
        }
    }
    let part1 = dancers.iter().join("");

    (part1, String::from("<not yet implemented>"))
}
