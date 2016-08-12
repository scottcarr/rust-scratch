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

fn compare(ex1: Example, ex2: Example, b: bool) -> bool {
    match (ex1, ex2) {
        (A, A) if b => true,
        (A, A) if !b => false,
        (B, B) => true,
        (C, C) => true,
        (D, D) => true,
        (E, E) => true,
        (F, F) => true,
        (G, G) => true,
        _ => false,
    }
}

fn main() {
    compare(A, A, false);
}
