// Lifetime ends; `v' becomes uninitialised in caller.
// Caller is returned a vector, which could be `v'.
fn take(v: Vec<i32>) -> Vec<i32>;

// Borrowing references from the caller:
// `w' is defined for the caller afterward and must not change
// `x' is defined for the caller afterward but could have changed
fn borrow(w: &Vec<i32>, x: &mut Vec<i32>);
