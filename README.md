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

use itertools::Itertools; // 0.9.0

fn main() {
    let some_iter = vec![1, 2, 3, 4, 5, 6].into_iter();

    for ( p, n, nn) in some_iter.tuples::<(_, _, _)>() {
        println!("{}--{}--{}", p, n, nn);
    }
}

use itertools::Itertools; // 0.9.0

fn main() {
    let some_iter = vec![1, 2, 3, 4, 5, 6].into_iter();

    for ( current, next) in some_iter.tuples::<(_, _)>() {
        println!("{}--{}", current, next);
    }

}


////////////////////////////////////////////
#[derive(Debug)]
pub struct PointArray{ 
    point: f32,
}


fn main() {
    let some_iter = vec![PointArray{point:1.0}, PointArray{point:2.0}, PointArray{point:3.0},PointArray{point:4.0}].into_iter();

    for ( current, next) in some_iter.tuples::<(_, _)>() {
        println!("{:?}--{:?}", current, next);
    }

}
