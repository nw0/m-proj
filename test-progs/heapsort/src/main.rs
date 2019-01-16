use std::env;

fn lf(i: usize) -> usize { i * 2 + 1 }
fn rt(i: usize) -> usize { i * 2 + 2 }
fn par(i: usize) -> usize { i / 2 }

fn sift(v: &mut Vec<i64>, start: usize, end: usize) {
    let mut root = start;
    while lf(root) <= end {
        let child = lf(root);
        let mut swap = root;

        if v[swap] < v[child] {
            swap = child;
        }
        if child + 1 <= end && v[swap] < v[child + 1] {
            swap = child + 1;
        }
        if swap == root {
            return;
        } else {
            let tmp = v[root];
            v[root] = v[swap];
            v[swap] = tmp;
            root = swap;
        }
    }
}


fn heapify(mut v: &mut Vec<i64>, i: usize) {
    let mut start: isize = par(i-1) as isize;
    while start >= 0 {
        sift(&mut v, start as usize, i-1);
        start -= 1;
    }
}

fn sort(mut v: &mut Vec<i64>) {
    let size = v.len();
    heapify(&mut v, size);

    let mut end = size - 1;
    while end > 0 {
        let tmp = v[end];
        v[end] = v[0];
        v[0] = tmp;
        end -= 1;
        sift(v, 0, end);
    }
}

fn main() {
    println!("Hello, world!");
    let mut vec = vec![53, 235, 45, 2, 65, -1];
    sort(&mut vec);
    println!("A built-in sorted vector: {:?}", vec);

    println!("Sorting the input arguments...");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut iv: Vec<i64> = (&args[1..]).iter().map(|s| s.parse::<i64>().unwrap()).collect();
        sort(&mut iv);
        println!("{:?}", iv);
    } else {
        println!("No input given.");
    }
}
