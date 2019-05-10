impl<T> [T] {
    pub fn repeat(&self, n: usize) -> Vec<T> where T: Copy {
        if n == 0 { return Vec::new(); }

        let mut buf = Vec::with_capacity(self.len() * n);
        buf.extend(self);
        {
            let mut m = n >> 1;
            while m > 0 {
                unsafe {
                    ptr::copy_nonoverlapping(
                        buf.as_ptr(),
                        (buf.as_mut_ptr() as *mut T).add(buf.len()),
                        buf.len(),
                    );
                    let buf_len = buf.len();
                    buf.set_len(buf_len * 2);
                }
                m >>= 1;
            }
        }
        // omitted: copy into the remainder of the vector
    }
}