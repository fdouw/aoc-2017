use itertools::Itertools;

use crate::lib_aoc::itertools::ChunkReduceIterator;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let lengths = input
        .trim_end()
        .split(',')
        .map(|l| l.parse::<usize>().unwrap());
    let mut list: Vec<_> = (0..256).collect();
    let mut pos = 0;
    let mut skip_size = 0;

    for len in lengths {
        let mut a = pos;
        let mut b = pos + len - 1; // -1 because b is inclusive
        while a < b {
            let tmp = list[a % 256];
            list[a % 256] = list[b % 256];
            list[b % 256] = tmp;
            a += 1;
            b -= 1;
        }
        pos += len + skip_size;
        skip_size += 1;
    }
    let part1 = list[0] * list[1];

    // Part 2
    let lengths: Vec<_> = input
        .trim()
        .as_bytes()
        .into_iter()
        .map(|c| *c as usize)
        .chain(vec![17, 31, 73, 47, 23])
        .collect();
    let mut list: Vec<u32> = (0..256).collect();
    let mut pos = 0;
    let mut skip_size = 0;

    for _round in 0..64 {
        for len in lengths.iter() {
            let len = len;
            let mut a = pos;
            let mut b = pos + len - 1; // -1 because b is inclusive
            while a < b {
                let tmp = list[a % 256];
                list[a % 256] = list[b % 256];
                list[b % 256] = tmp;
                a += 1;
                b -= 1;
            }
            pos += len + skip_size;
            skip_size += 1;
        }
    }

    let part2 = list
        .into_iter()
        .chunk_reduce(16, |a, b| a ^ b)
        .map(|n| format!("{:02x}", n))
        .join("");

    (part1.to_string(), part2)
}
