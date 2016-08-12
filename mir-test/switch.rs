fn main() {
}

fn foo(n: usize) {
    match n {
        1 |
        2 | 
        _ => bar()
    };
}

fn bar() {
    println!("hello world!");
}
