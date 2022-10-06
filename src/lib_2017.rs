use crate::lib_aoc::itertools::*;

pub fn knot_hash(bytes: &[u8]) -> Vec<u8> {
    let lengths: Vec<_> = bytes
        .into_iter()
        .map(|c| *c as usize)
        .chain(vec![17, 31, 73, 47, 23])
        .collect();
    let mut list: Vec<u8> = (0..=255).collect();
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

    list.into_iter().chunk_reduce(16, |a, b| a ^ b).collect()
}
