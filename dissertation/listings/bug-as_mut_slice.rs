impl<T> IntoIter<T> {
    pub fn as_mut_slice(&self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.ptr as *mut T, self.len())
        }
    }
}