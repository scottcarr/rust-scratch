pub fn test(a: &[u64; 8]) -> [u64; 8] {
    let b = *a;
    let c = b;
    let d = c;
    d
}

fn main() {}
