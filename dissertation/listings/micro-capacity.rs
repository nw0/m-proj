fn main() {
    use alloc::collections::VecDeque;

    let mut deque = VecDeque::with_capacity(31);
    deque.push_front(5);
    println!("{:?}", deque);                    // `[5]'
    println!("{}, {}", deque.head, deque.tail); // `0, 31'

    deque.reserve(30);
    println!("{}, {}", deque.head, deque.tail); // `32, 31'

    deque.push_back(6);                         // CHERI: length violation!
    println!("{:?}", deque);                    // `[5, 0]' <- not [5, 6]
    println!("{}, {}", deque.head, deque.tail); // `1, 31'
}
