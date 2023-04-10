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

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures
pub fn inverse(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = extended_gcd(x, n);
    (g == 1).then_some((x % n + n) % n)
}

// and finally https://en.wikipedia.org/wiki/Chinese_remainder_theorem
// this only works because everything is pairwise coprime
pub fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod: i64 = modulii.iter().product();

    let mut sum = 0;
    let pairs = residues.iter().zip(modulii);

    for (&residue, &modulus) in pairs {
        let p = prod / modulus;
        sum += residue * inverse(p, modulus)? * p
    }

    Some(sum % prod)
}
