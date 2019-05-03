impl<T> VecDeque<T> {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap() - 1
    }
}