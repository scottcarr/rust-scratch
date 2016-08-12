
struct Baz {
    x: usize,
    y: f32,
    z: bool,
}

fn bar(a: usize) -> Baz {
    Baz { x: a, y: 0.0, z: false }
}

fn main() {}
