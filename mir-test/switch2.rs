enum E {
    A,
    B,
    C
}

fn main() {
}

fn foo(n: E) {
    match n {
        E::A |
        E::B | 
        E::C => bar()
    };
}

fn bar() {
    println!("hello world!");
}
