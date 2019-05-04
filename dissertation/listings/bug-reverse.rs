impl<I> RandomAccessIterator for Rev<I>
where
    I: DoubleEndedIterator + RandomAcc,
{
    #[inline]
    fn idx(&mut self, index: usize) -> Option<<I as Iterator>::Item> {
        let amt = self.indexable();
        self.iter.idx(amt - index - 1)
    }
}
