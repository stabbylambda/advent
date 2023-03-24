pub fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x.abs();
    let mut y = y.abs();

    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }

    x
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}
