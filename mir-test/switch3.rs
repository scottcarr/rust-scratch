enum E {
    C,
}

fn main() {
}

fn foo(n: E) {
    match n {
        E::C => bar()
    };
}

fn bar() {
    println!("hello world!");
}
