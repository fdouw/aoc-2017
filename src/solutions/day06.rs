use std::collections::HashMap;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let mut membanks = input
        .split_ascii_whitespace()
        .map(|x| u64::from_str_radix(x, 10).unwrap())
        .collect::<Vec<_>>();
    let n_banks = membanks.len();

    let mut seen = HashMap::new();
    let mut cycle = 0;
    let loop_size;
    loop {
        // Hash the membanks and check if we have seen it before
        let mut hash = 0;
        for n in &membanks {
            hash = (hash << 4) + n;
        }
        if let Some(prev) = seen.insert(hash, cycle) {
            loop_size = cycle - prev;
            break;
        }

        // Find the index of the max value
        let mut max_idx = 0;
        for (idx, val) in (&membanks).into_iter().enumerate() {
            if val > &membanks[max_idx] {
                max_idx = idx;
            }
        }

        // Redistribute
        let points = membanks[max_idx];
        membanks[max_idx] = 0;
        let mut idx = max_idx + 1;
        for _ in 0..points {
            membanks[idx % n_banks] += 1;
            idx += 1;
        }

        cycle += 1;
    }

    (cycle.to_string(), loop_size.to_string())
}
