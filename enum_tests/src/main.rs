#![allow(dead_code)]

use self::Team::*;

enum Team {
    Tigers,
    Giants,
    Rays,
    Dodgers,
}

fn foo(x: Team, y: bool) {
    match x {
        Tigers if y => println!("matched tigers"),
        Tigers if !y => println!("matched not tigers"),
        Giants => println!("matched giants"),
        Rays => println!("matched rays"),
        Dodgers => println!("matched dodgers"),
    }
}

fn main() {
    println!("Hello, world!");
    foo(Tigers, true);
}
