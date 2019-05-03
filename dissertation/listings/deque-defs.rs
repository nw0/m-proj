impl<T> VecDeque<T> {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap() - 1
    }

    pub fn push_back(&mut self, value: T) {
        self.grow_if_necessary();

        let head = self.head;                       // PRE: 0 <= head < len
        self.head = self.wrap_add(self.head, 1);
        unsafe { ptr::write(self.ptr().add(head), value) }  // unchecked
    }
}
