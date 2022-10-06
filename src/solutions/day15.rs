use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    #[allow(unused)]
    let (a_start, b_start) = input
        .trim()
        .lines()
        .map(|l| l[24..].parse::<u64>().unwrap())
        .next_tuple()
        .unwrap();

    // Test values in problem statement
    // let (a_start, b_start) = (65, 8921);

    // Given in problem statement
    let factor_a: u64 = 16807;
    let factor_b: u64 = 48271;
    let divisor = 2147483647;

    let mut a_val = a_start;
    let mut b_val = b_start;

    let mut match_count_part1 = 0;
    for _ in 0..40_000_000 {
        a_val = (a_val * factor_a) % divisor;
        b_val = (b_val * factor_b) % divisor;
        if a_val & 0b11111111_11111111 == b_val & 0b11111111_11111111 {
            match_count_part1 += 1;
        }
    }

    let mut gen_a = Generator {
        factor: factor_a,
        bit_filter: 0b0011,
        value: a_start,
    };

    let mut gen_b = Generator {
        factor: factor_b,
        bit_filter: 0b0111,
        value: b_start,
    };

    let mut match_count_part2 = 0;
    for _ in 0..5_000_000 {
        a_val = gen_a.next();
        b_val = gen_b.next();
        if a_val & 0b11111111_11111111 == b_val & 0b11111111_11111111 {
            match_count_part2 += 1;
        }
    }

    (match_count_part1.to_string(), match_count_part2.to_string())
}

struct Generator {
    factor: u64,
    bit_filter: u64,
    value: u64,
}
impl Generator {
    fn next(&mut self) -> u64 {
        loop {
            self.value = (self.value * self.factor) % 0x7fffffff; // divisor given in prob. statement
            if self.value & self.bit_filter == 0 {
                break;
            }
        }
        self.value
    }
}
