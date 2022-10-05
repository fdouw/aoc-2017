pub fn solve(input: String, _verbose: bool) -> (String, String) {
    // Part 1
    let mut code: Vec<_> = input
        .lines()
        .enumerate()
        .map(|x| x.0 as i64 + x.1.parse::<i64>().unwrap())
        .collect();

    let length = code.len() as i64;
    let mut idx = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let nxt = code[idx];
        code[idx] += 1;
        if nxt < 0 || length <= nxt {
            break;
        } else {
            idx = nxt as usize;
        }
    }

    // Part 2
    let mut code: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut idx = 0;
    let mut steps2 = 0;
    while idx >= 0 && idx < length {
        steps2 += 1;
        let delta = code[idx as usize];
        if delta >= 3 {
            code[idx as usize] -= 1;
        } else {
            code[idx as usize] += 1;
        }
        idx += delta;
    }

    (steps.to_string(), steps2.to_string())
}
