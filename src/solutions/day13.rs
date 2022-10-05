use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let _test_input = "0: 3\n1: 2\n4: 4\n6: 4";

    let mut total_severity = 0;
    for line in input.trim().lines() {
        let (layer, depth) = line
            .split(": ")
            .map(|n| n.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap();

        // Scanners move up and down, so their travel path is twice the scan depth minus two (they don't stay on the endpoints)
        let travel_path = depth * 2 - 2;
        // We reach layer n at time = n picoseconds
        if layer % travel_path == 0 {
            total_severity += depth * layer;
        }
    }

    // Part 2
    // Move all the scanners along their path according to their layer;
    // because by the time we get there, the scanner will have moved to that point
    let scanners: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            let (layer, depth) = line
                .split(": ")
                .map(|n| n.parse::<u64>().unwrap())
                .next_tuple()
                .unwrap();
            let position = layer;
            let path_length = 2 * depth - 2;
            (position, path_length)
        })
        .collect();

    // There should be a smarter way to solve this system, but do brute force for now
    let mut part2 = 0;
    for wait in 0.. {
        if scanners.iter().all(|(pos, path)| (pos + wait) % path != 0) {
            part2 = wait;
            break;
        }
    }

    return (total_severity.to_string(), part2.to_string());
}
