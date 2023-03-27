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

// pretty much ripped directly from https://github.com/rust-lang/rust/pull/88582/files#diff-dd440fe33121a785308d5cde98a1ab79b0b285d27bb29eaa9800e180870e16a6R1809
pub const fn div_ceil(lhs: i64, rhs: i64) -> i64 {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if (r > 0 && rhs > 0) || (r < 0 && rhs < 0) {
        d + 1
    } else {
        d
    }
}
