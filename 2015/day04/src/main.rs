use common::{answer, read_input};
use md5::{Digest, Md5};

fn main() {
    let input = read_input!();

    answer!(problem1(input));
    answer!(problem2(input));
}

fn hash<F>(input: &str, f: F) -> u32
where
    F: Fn(&[u8]) -> i32,
{
    let input = input.as_bytes();

    for i in 0u32.. {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.update(&i.to_string());

        let output = hasher.finalize();

        if f(&output) == 0 {
            return i;
        }
    }

    unreachable!()
}

fn problem1(input: &str) -> u32 {
    hash(input, |output| {
        output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32
    })
}

fn problem2(input: &str) -> u32 {
    hash(input, |output| {
        output[0] as i32 + output[1] as i32 + output[2] as i32
    })
}

#[cfg(test)]
mod test {

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let result = problem1("abcdef");
        assert_eq!(result, 609043)
    }

    #[test]
    #[ignore = "too slow"]
    fn second() {
        let result = problem2("abcdef");
        assert_eq!(result, 6742839)
    }
}
