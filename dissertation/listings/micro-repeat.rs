fn main() {
    use core::ptr::copy_nonoverlapping;

    let s: [i64; 3] = [1, 2 ,3];
    let reps = 6148914691236517207;             // (2 ** 64 + 5) / 3

    // Simulate usize == u64 on CHERI, where usize == u128
    let mut buf = Vec::with_capacity((s.len() * reps) as u64 as usize);
    prntf!("capacity: %d\n", buf.capacity() as u64);    // 5
    buf.extend(&s);

    let mut v: Vec<i64> = Vec::new();
    v.extend(&s);                               // Manipulate allocation to
    v[0] = -1;                                  //  ensure the buffers are
    v.push(-4);                                 //  allocated nearby
    for x in &v { prntf!("%d ", *x); } prntf!("\n");    // -1 2 3 -4

    {
        let mut m = 8;      // 8 <= n >> 1; copy enough times to reach `v'
        while m > 0 {
            unsafe {
                copy_nonoverlapping(            // generates memcpy
                    buf.as_ptr(),               // out of bounds on 1st iter
                    (buf.as_mut_ptr() as *mut i64).add(buf.len()),
                    buf.len()
                );                              // CHERI: length violation
                let buf_len = buf.len();
                buf.set_len(buf_len * 2);
            }
            m >>= 1;
        }
    }
    for x in &v { prntf!("%d ", *x); } prntf!("\n");    // 1 2 3 1
}
