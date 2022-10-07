use itertools::Itertools;
use regex::Regex;

enum DanceMove {
    Spin,
    Exchange,
    Partner,
}

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let re = Regex::new("^(?P<t>.)(?P<a>[^/]+)(/(?P<b>.+))?$").unwrap();
    let mut dancers = (97..113).collect::<Vec<u8>>(); // numeric values of ascii characters up to p (incl.)
    let mut rules = Vec::new();
    let mut configs = Vec::new();

    // Test data
    // let input = "s1,x3/4,pe/b";
    // let mut dancers = (97..102).collect::<Vec<u8>>();

    // Encode the rules
    for dance_move in input.trim().split(",") {
        let step = re.captures(dance_move).unwrap();
        match step.name("t").unwrap().as_str() {
            "s" => {
                // Spin the line of programs
                let amount = step.name("a").unwrap().as_str().parse::<usize>().unwrap();
                rules.push((DanceMove::Spin, amount, 0));
            }
            "x" => {
                // Exchange 2 indices
                let a = step.name("a").unwrap().as_str().parse::<usize>().unwrap();
                let b = step.name("b").unwrap().as_str().parse::<usize>().unwrap();
                rules.push((DanceMove::Exchange, a, b));
            }
            "p" => {
                // Partner 2 programs (ie, swap names)
                let a = step.name("a").unwrap().as_str().chars().nth(0).unwrap() as usize;
                let b = step.name("b").unwrap().as_str().chars().nth(0).unwrap() as usize;
                rules.push((DanceMove::Partner, a, b));
            }
            x => panic!("Invalid instruction: {x}"),
        }
    }

    // Put the initial state in the memory
    configs.push(dancers.iter().map(|c| char::from(*c)).join(""));

    // Repeat the dance until the configuration of the programs (dancers) repeats
    let mut cycle_length = 0;
    for round in 0.. {
        for rule in rules.iter() {
            match rule.0 {
                DanceMove::Spin => dancers.rotate_right(rule.1),
                DanceMove::Exchange => {
                    let tmp = dancers[rule.1];
                    dancers[rule.1] = dancers[rule.2];
                    dancers[rule.2] = tmp;
                }
                DanceMove::Partner => {
                    let mut changed = 0u8;
                    for n in dancers.iter_mut() {
                        if *n == rule.1 as u8 {
                            *n = rule.2 as u8;
                            changed += 1;
                        } else if *n == rule.2 as u8 {
                            *n = rule.1 as u8;
                            changed += 1;
                        }
                        if changed == 2 {
                            break;
                        }
                    }
                }
            }
        }
        // Keep track of the configurations in order, to compute the final state
        configs.push(dancers.iter().map(|c| char::from(*c)).join(""));

        // When we see the initial state, the pattern repeats
        if configs.last().unwrap() == "abcdefghijklmnop" {
            cycle_length = round + 1;
            break;
        }
    }

    // The pattern repeats; we only need the residual dances to work out the final config
    let residue = 1_000_000_000 % cycle_length;
    let part1 = &configs[1];
    let part2 = &configs[residue];

    (part1.to_owned(), part2.to_owned())
}
