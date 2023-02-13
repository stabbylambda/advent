use crypto::{digest::Digest, md5::Md5};

fn main() {
    let input = include_str!("../input.txt");

    let answer = problem1(input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(input);
    println!("problem 2 answer: {answer}");
}

fn hash<F>(input: &str, f: F) -> u32
where
    F: Fn(&[u8]) -> i32,
{
    let input = input.as_bytes();
    let mut hasher = Md5::new();

    for i in 0u32.. {
        hasher.input(input);
        hasher.input_str(&i.to_string());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        if f(&output) == 0 {
            return i;
        }
        hasher.reset();
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
    fn second() {
        let result = problem2("abcdef");
        assert_eq!(result, 6742839)
    }
}
