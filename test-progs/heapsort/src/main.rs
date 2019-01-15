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
    let mut start = par(i-1);
    while start >= 0 {
        sift(&mut v, start, i-1);
        if start == 0 {
            break;
        }
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
    println!("{:?}", vec);
}
