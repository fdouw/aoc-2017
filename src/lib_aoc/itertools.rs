pub struct ChunkReduce<I, F>
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

pub trait ChunkReduceIterator<F>: IntoIterator + Sized
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
