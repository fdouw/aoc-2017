use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // Run the instructions (brute force)
    let mut registers = Registers::new();
    let mut part2_highest = 0; // All registers are initially 0
    for line in input.lines() {
        let rule: Vec<_> = line.splitn(5, " ").collect();
        let name = rule[0];
        let increment = match rule[1] {
            "inc" => rule[2].parse::<i64>().unwrap(),
            "dec" => rule[2].parse::<i64>().unwrap() * -1,
            _ => panic!("Invalid instruction: '{}'", rule[1]),
        };
        let condition = rule[4];
        let new = registers.increment_if(name, increment, condition);
        part2_highest = part2_highest.max(new);
    }

    // Find the largest value in the registers
    let part1 = registers.largest();

    (part1.to_string(), part2_highest.to_string())
}

struct Registers<'a> {
    registers: HashMap<&'a str, i64>,
}

impl<'a> Registers<'a> {
    fn new() -> Self {
        Registers {
            registers: HashMap::new(),
        }
    }
    fn get(&self, name: &str) -> i64 {
        *self.registers.get(name).unwrap_or(&0)
    }
    fn set(&mut self, name: &'a str, value: i64) {
        self.registers.insert(name, value);
    }
    fn increment_if(&mut self, name: &'a str, increment: i64, condition: &str) -> i64 {
        // Returns the value in register `name` after this operation
        let old = self.get(name);
        if self.test(condition) {
            self.set(name, old + increment);
            old + increment
        } else {
            old
        }
    }
    fn test(&self, condition: &str) -> bool {
        let cond: Vec<&str> = condition.trim().split_ascii_whitespace().collect();
        let current = self.get(cond[0]);
        let other: i64 = cond[2].parse().unwrap();
        match cond[1] {
            "==" => current == other,
            "!=" => current != other,
            ">" => current > other,
            ">=" => current >= other,
            "<" => current < other,
            "<=" => current <= other,
            _ => panic!("Invalid comparison: {}", cond[1]),
        }
    }
    fn largest(&self) -> i64 {
        *self.registers.values().max().unwrap()
    }
}
