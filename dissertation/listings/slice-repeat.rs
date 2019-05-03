impl<T> [T] {
    pub fn repeat(&self, n: usize) -> Vec<T> where T: Copy {
        // omitted: trivial n = 0 case
        let mut buf = Vec::with_capacity(self.len() * n);

        buf.extend(self);
        {
            let mut m = n >> 1;
            while m > 0 {
                unsafe {
                    // memcpy existing elements to double the length
                }
            }
        }
        // omitted: copy into the remainder of the vector
    }
}