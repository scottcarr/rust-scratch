#![allow(dead_code)]

use self::Example::*;

enum Example {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn compare(ex1: Example, b: bool) -> bool {
    match ex1 {
        A if b => true,
        A if !b => true,
        B => true,
        C => true,
        D => true,
        E => true,
        F => true,
        G => false,
    }
}


fn main() {
    compare(A, false);
}
