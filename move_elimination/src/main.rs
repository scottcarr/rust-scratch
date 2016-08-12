struct Foo {
    x: Vec<usize>,
    y: Vec<usize>,
}

fn main() {
    let mut x = 0;
    let mut y = 0;
    {
        let p = &mut y;
        x = x + {*p = 22; 3 };
    }
    println!("x: {}", x);
    println!("y: {}", y);
    let f = Foo {
        x: vec![1,2,3],
        y: vec![4,5,6],
    };
    println!("f.x[0]: {}", f.x[0]);
    println!("f.y[0]: {}", f.y[0]);
}
