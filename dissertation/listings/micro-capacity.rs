fn main() {
    use alloc::collections::VecDeque;

    let mut deque = VecDeque::with_capacity(31);
    deque.push_front(5);
    for x in &deque { prntf!("%d ", *x); } prntf!("\n");        // `5'
    prntf!("%d, %d", deque.head as u64, deque.tail as u64);     // `0, 31'

    deque.reserve(30);                          // head should not change
    prntf!("%d, %d", deque.head as u64, deque.tail as u64);     // `32, 31'

    deque.push_back(6);                         // CHERI: length violation!
    for x in &deque { prntf!("%d ", *x); }      // `5 0' <- not `5 6'!
    prntf!("\n%d, %d", deque.head as u64, deque.tail as u64);   // `1, 31'
}
