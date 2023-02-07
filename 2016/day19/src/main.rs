fn main() {
    let input = 3_012_210;
    println!("Solution 1: {}", problem1(input));
    println!("Solution 2: {}", problem2(input as u32));
}

// return a number with only the most significant bit set
fn msb(n: i32) -> i32 {
    let mut count: i32 = 0;
    let mut num = n;
    while num > 0 {
        count += 1;
        num >>= 1;
    }

    1 << (count - 1)
}

fn problem1(input: i32) -> i32 {
    // https://en.wikipedia.org/wiki/Josephus_problem#Bitwise
    !msb(input * 2) & ((input << 1) | 1)
}

fn problem2(input: u32) -> u32 {
    let mut i = 1;

    while i * 3 < input {
        i *= 3;
    }

    input - i
}

#[cfg(test)]
mod test {
    use crate::problem1;
    #[test]
    fn first() {
        assert_eq!(problem1(5), 3)
    }
}
