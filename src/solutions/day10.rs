use itertools::Itertools;

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

struct ChunkReduce<I, F>
where
    I: IntoIterator,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    iter: I,
    chunk_size: usize,
    func: F,
}

impl<I, F> ChunkReduce<I, F>
where
    I: IntoIterator,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    fn new(iter: I, chunk_size: usize, func: F) -> Self {
        Self {
            iter,
            chunk_size,
            func,
        }
    }
}

impl<I: Iterator, F> Iterator for ChunkReduce<I, F>
where
    F: FnMut(I::Item, I::Item) -> I::Item + Copy,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let mut accum = self.iter.next()?;
        for _ in 1..(self.chunk_size) {
            let next = match self.iter.next() {
                Some(v) => v,
                None => break,
            };
            accum = (self.func)(accum, next);
        }
        Some(accum)
    }
}

trait ChunkReduceIterator<F>: IntoIterator + Sized
where
    F: FnMut(
        <Self as IntoIterator>::Item,
        <Self as IntoIterator>::Item,
    ) -> <Self as IntoIterator>::Item,
{
    fn chunk_reduce(self, chunk_size: usize, func: F) -> ChunkReduce<Self, F> {
        ChunkReduce::new(self, chunk_size, func)
    }
}

impl<F, I: Iterator> ChunkReduceIterator<F> for I where
    F: FnMut(
        <Self as IntoIterator>::Item,
        <Self as IntoIterator>::Item,
    ) -> <Self as IntoIterator>::Item
{
}
