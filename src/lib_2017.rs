use itertools::Itertools;

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

#[allow(dead_code)]
pub fn knot_hash_hex(bytes: &[u8]) -> String {
    knot_hash(bytes)
        .into_iter()
        .map(|n| format!("{:02x}", n))
        .join("")
}

#[cfg(test)]
mod knot_hash {
    use crate::lib_2017::*;
    use pretty_assertions::assert_eq;

    // Tests as given on the problem page: https://adventofcode.com/2017/day/10
    #[test]
    fn test_hex_empty() {
        assert_eq!(
            knot_hash_hex("".as_bytes()),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
    }
    #[test]
    fn test_hex_aoc() {
        assert_eq!(
            knot_hash_hex("AoC 2017".as_bytes()),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
    }
    #[test]
    fn test_hex_123() {
        assert_eq!(
            knot_hash_hex("1,2,3".as_bytes()),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
    }
    #[test]
    fn test_hex_124() {
        assert_eq!(
            knot_hash_hex("1,2,4".as_bytes()),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
