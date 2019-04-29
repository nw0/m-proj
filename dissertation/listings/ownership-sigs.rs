// Lifetime ends; v becomes uninitialised in caller.
fn take_ownership(v: Vec<i32>);

// Lifetime ends; v becomes uninitialised in caller.
// Caller receives a vector, which may be the original vector.
fn take_and_return(v: Vec<i32>) -> Vec<i32>;

// Receiving an immutable reference.
// The vector is still defined for the caller afterward.
fn borrow(v: &Vec<i32>);

// Receiving a mutable reference.
// The vector is still defined for the caller afterward; it may be changed.
fn borrow_mutably(v: &mut Vec<i32>);