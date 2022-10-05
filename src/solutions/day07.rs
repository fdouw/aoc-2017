use itertools::{self, Itertools};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut programs = HashSet::new();
    let mut children = HashSet::new();
    let re = Regex::new(r" -> |,? ").unwrap();

    // Read in the programs and the children that they refer to
    // The root will be the only program that is not been refered to
    for line in input.lines() {
        let mut fields = re.split(line);
        programs.insert(fields.next().unwrap().trim());
        let mut fields = fields.skip(1); // skip weight
        while let Some(name) = fields.next() {
            children.insert(name);
        }
    }
    let root_name = programs.difference(&children).next().unwrap();

    // Part 2
    // Generate list of programs
    let mut programs = HashMap::new();
    let parentheses: &[_] = &['(', ')'];
    for line in input.lines() {
        let mut fields = re.split(line);
        let name = fields.next().unwrap().trim();
        let weight = fields
            .next()
            .unwrap()
            .trim_matches(parentheses)
            .parse()
            .unwrap();
        let mut program = Program::new(name, weight);
        while let Some(child) = fields.next() {
            program.children.push(child);
        }
        programs.insert(name, program);
    }

    let mut current = root_name;
    let mut target_weight = programs.get(root_name).unwrap().get_weight(&programs); //Assume root weight is correct
    loop {
        let mut weight_counts: Vec<_>;
        let program = &programs.get(current).unwrap();
        if program.children.len() == 0 {
            panic!("Reached the leaves without finding the error! Leave: '{current}'");
        }

        // Collect the weights of the children (subtowers)
        weight_counts = program
            .children
            .iter()
            .map(|n| (n, programs.get(n).unwrap().get_weight(&programs)))
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .dedup_by_with_count(|a, b| a.1 == b.1)
            .collect();

        if weight_counts.len() == 1 {
            // All subtowers have the same weight, meaning the error is in the current program
            // Correct tower weight is stored in `target_weight`; subtract subtowers to get the correct program weight
            target_weight -= weight_counts[0].0 as i64 * weight_counts[0].1 .1;
            break;
        } else {
            weight_counts.sort_by(|a, b| a.0.cmp(&b.0));
            let invalid = weight_counts[0].1;
            target_weight = weight_counts[1].1 .1;
            current = invalid.0;
        }
    }

    (root_name.to_string(), target_weight.to_string())
}

struct Program<'p> {
    _name: &'p str,
    weight: i64,
    tower_weight: Option<i64>,
    children: Vec<&'p str>,
}

impl Program<'_> {
    fn new(name: &str, weight: i64) -> Program {
        Program {
            _name: name,
            weight,
            tower_weight: None,
            children: Vec::new(),
        }
    }

    fn get_weight(&self, programs: &'_ HashMap<&str, Program>) -> i64 {
        match self.tower_weight {
            Some(w) => w,
            None => {
                // TODO: memoization
                let mut w = self.weight;
                for child in &self.children {
                    w += programs.get(child).unwrap().get_weight(programs);
                }
                w
            }
        }
    }
}
