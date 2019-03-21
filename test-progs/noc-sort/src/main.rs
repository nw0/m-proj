#![feature(no_core)]
#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]

#![no_core]

extern crate libcore_cheri as core;

#[cfg(target_os = "freebsd")]
#[link(name = "c")]
extern {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn lf(i: usize) -> usize { i * 2 + 1 }
fn rt(i: usize) -> usize { i * 2 + 2 }
fn par(i: usize) -> usize { i / 2 }

fn sift(v: &mut [i32], start: usize, end: usize) {
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

fn heapify(mut v: &mut [i32], i: usize) {
    let mut start: isize = par(i-1) as isize;
    while start >= 0 {
        sift(&mut v, start as usize, i-1);
        start -= 1;
    }
}

fn sort(mut v: &mut [i32]) {
    let sz = v.len();
    heapify(&mut v, sz);

    let mut end = sz - 1;
    while end > 0 {
        let tmp = v[end];
        v[end] = v[0];
        v[0] = tmp;
        end -= 1;
        sift(v, 0, end);
    }
}

fn main() {
    let mut arr: [i32; 6] = [53, 235, 45, 2, 65, -1];
    sort(&mut arr);

    unsafe {
        printf(b"Sorted? %d %d %d %d %d %d\n\0" as *const u8, arr[0], arr[1], arr[2], arr[3], arr[4], arr[5]);
    }
}
