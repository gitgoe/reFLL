# rustylogic
rusty version of eFLL


fn foo(n: usize) {
    if let Some(_) = dbg!(n.checked_sub(4)) {
        // ...
    }
}

use std::mem;

let mut x = 5;
let mut y = 42;

mem::swap(&mut x, &mut y);

assert_eq!(42, x);
assert_eq!(5, y);

let v = vec![1, 2, 3];

drop(v); // explicitly drop the vector




cargo test test_build -- --nocapture


