pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let data = input.trim().lines();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in data {
        let mut nums: Vec<_> = line
            .split_ascii_whitespace()
            .map(|w| str::parse::<u32>(w).unwrap())
            .collect();
        nums.sort();
        part1 += nums.last().unwrap() - nums.first().unwrap();

        'part2: for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[j] % nums[i] == 0 {
                    part2 += nums[j] / nums[i];
                    break 'part2;
                }
            }
        }
    }

    (format!("{part1}"), format!("{part2}"))
}
