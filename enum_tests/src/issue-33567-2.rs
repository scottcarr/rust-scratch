#![allow(dead_code)]

fn compare(ex1: u32, ex2: u32) -> bool {
    match (ex1, ex2) {
        (0, 0) => true,
        (1, 1) => true,
        (2, 2) => true,
        (3, 3) => true,
        (4, 4) => true,
        (5, 5) => true,
        (6, 6) => true,
        _ => false,
    }
}

fn main() {
}
