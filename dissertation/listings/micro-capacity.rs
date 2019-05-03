fn main() {
    use alloc::collections::VecDeque;

    let mut deque = VecDeque::with_capacity(31);
    deque.push_front(5);

    for x in &deque {
        print_sp(*x);                           // `5'
    }
    unsafe { printf(b"\n\0" as *const u8); }
    print_sp(deque.head as i64);                // `0'
    print_sp(deque.tail as i64);                // `31'
    unsafe { printf(b"\n\0" as *const u8); }
    deque.reserve(30);
    print_sp(deque.head as i64);                // `32'
    print_sp(deque.tail as i64);                // `31'
    unsafe { printf(b"\n\0" as *const u8); }
    deque.push_back(6);                         // CHERI: length violation!
    for x in &deque {
        print_sp(*x);                           // `5 0'
    }
    unsafe { printf(b"\n\0" as *const u8); }
    print_sp(deque.head as i64);                // `1'
    print_sp(deque.tail as i64);                // `31'
    unsafe { printf(b"\n\0" as *const u8); }
}
