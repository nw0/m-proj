fn sum_iter(arr: &[i64]) -> i64 {
    let mut sum: i64 = 0;
    for x in r.iter() { sum += *x; }
    sum
}
fn sum_foreach(arr: &[i64]) -> i64 {
    let mut sum: i64 = 0;
    arr.iter().for_each(|x| sum += x);
    sum
}
fn sum_builtin(arr: &[i64]) -> i64 {
    arr.iter().sum()
}
fn sum_loop(arr: &[i64]) -> i64 {
    let mut sum: i64 = 0;
    let mut i: usize = 0;
    loop {
        if i >= arr.len() { break }
        sum += arr[i]; i += 1;
    }
    sum
}