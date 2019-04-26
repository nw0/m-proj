fn sum_checked(arr: &[i64]) -> i64 {
    let mut sum: i64 = 0;
    let mut i: usize = 0;
    loop {
        if i > arr.len() - 1 { break }
        sum += arr[i]; i += 1;
    }
    sum
}