use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let start_state = input.lines().next().unwrap().chars().nth(15).unwrap() as u8 - b'A';
    let step_limit: usize = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();

    // Assume the states are named A,B,C,... And that their definitions appear in order.
    // Then we can store them in an ordered vector
    let mut states = Vec::new();

    // Assume all state instructions follow the same format
    for state_data in input.trim().split("\n\n").skip(1) {
        let mut lines = state_data.trim().lines().skip(2);
        // let _name = lines.next().unwrap().chars().nth(9).unwrap() as u8 - b'A';
        // lines.next();
        // If the current value is 0:
        let write_value_0 = lines.next().unwrap().chars().nth(22).unwrap() == '1';
        let move_right_0 = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            == "right.";
        let next_state_0 = lines.next().unwrap().chars().nth(26).unwrap() as u8 - b'A';
        lines.next();
        // If the current value is 1:
        let write_value_1 = lines.next().unwrap().chars().nth(22).unwrap() == '1';
        let move_right_1 = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            == "right.";
        let next_state_1 = lines.next().unwrap().chars().nth(26).unwrap() as u8 - b'A';
        states.push((
            (
                write_value_0,
                if move_right_0 { 1 } else { -1 },
                next_state_0 as usize,
            ),
            (
                write_value_1,
                if move_right_1 { 1 } else { -1 },
                next_state_1 as usize,
            ),
        ));
    }

    // Create an empty tape and run the program
    let mut tape = HashMap::new();
    let mut pointer = 0isize;

    let mut state_idx = start_state as usize;

    for _step in 0..step_limit {
        let value = tape.entry(pointer).or_insert(false);
        let instruction = if *value {
            states[state_idx].1
        } else {
            states[state_idx].0
        };
        *value = instruction.0;
        pointer += instruction.1;
        state_idx = instruction.2;
    }

    // Compute the checksum
    let checksum = tape.values().fold(0, |acc, x| acc + *x as u32);

    (checksum.to_string(), String::from("<no part 2 for day 25>"))
}
