pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let steps = input.parse::<usize>().unwrap();
    let mut ring = vec![0];

    // Test value
    // let steps = 3;

    // Part 1
    for n in 1..=2017 {
        let len = ring.len();
        ring.rotate_left(steps % len);
        ring.push(n);
    }
    let part1 = ring[0];

    // Part 2
    let mut val = 1; // value at index 1 (directly after 0)
    let mut idx = 1; // index where we insert the current value
    for len in 2..=50_000_000 {
        // len is length of the array and the next value to insert
        idx = (idx + steps) % len + 1; // index to insert the next value, +1 because we insert after
        if idx == 1 {
            val = len;
        }
    }
    let part2 = val;

    (part1.to_string(), part2.to_string())
}
