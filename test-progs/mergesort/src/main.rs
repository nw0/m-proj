use std::cmp::min;

fn merge(v: &mut Vec<i64>, w: &mut Vec<i64>, l: usize, r: usize, end: usize) {
    let mut i = l;
    let mut j = r;
    let mut k = l;

    while k < end {
        if i < r && (j >= end || v[i] <= v[j]) {
            w[k] = v[i];
            i += 1;
        } else {
            w[k] = v[j];
            j += 1;
        }
        k += 1;
    }
}

fn sorted (v: &Vec<i64>) -> Vec<i64> {
    let mut w0 = v.to_vec();
    let mut w1 = vec![0; v.len()];

    let mut block = 1;
    let mut i = 0;
    let mut a = 0;
    while block < v.len() {
        i = 0;
        while i < v.len() {
            if a == 0 {
                merge(&mut w0, &mut w1, i, min(i + block, v.len()), min(i + 2 * block, v.len()));
            } else {
                merge(&mut w1, &mut w0, i, min(i + block, v.len()), min(i + 2 * block, v.len()));
            }
            i += 2 * block;
        }
        a = 1 - a;
        block *= 2;
    }
    w0
}

fn main() {
    println!("Hello, world!");
    let mut arr = [73, 21, 2, 8, 1, 3];
    arr.sort();
    println!("{:?}", arr);
    let mut vec = vec![53, 235, 45, 2, 65, -1];
    vec.sort();
    println!("{:?}", vec);
    vec.push(12);
    println!("{:?}", sorted(&vec));
    println!("{}", 5f32.sqrt());
}
