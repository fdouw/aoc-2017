pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut data = input.trim().chars();
    let mut prev = data.next().unwrap().to_digit(10).unwrap();
    let first = prev;
    let mut sum = 0;
    for c in data {
        let x = c.to_digit(10).unwrap();
        if x == prev {
            sum += x;
        }
        prev = x;
    }
    if prev == first {
        sum += prev;
    }
    let part1 = sum;

    // Part 2
    let data: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let half = data.len() / 2;
    let mut sum = 0;
    for i in 0..half {
        if data[i] == data[i + half] {
            sum += data[i];
        }
    }
    let part2 = sum * 2;

    (format!("{part1}"), format!("{part2}"))
}
