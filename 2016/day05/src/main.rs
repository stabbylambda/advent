use common::{answer, read_input};
use md5::{Digest, Md5};
use std::fmt::Write;

fn main() {
    let input = read_input!();

    answer!(problem1(input));
    answer!(problem2(input));
}

fn hash(md5: &Md5, start: u128) -> (u128, [u8; 16]) {
    for i in start.. {
        let mut hasher = md5.clone();
        hasher.update(i.to_string());

        let output = hasher.finalize();

        let valid = (output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32) == 0;
        if valid {
            return (i + 1, output.into());
        }
    }

    unreachable!()
}

fn problem1(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.update(input);

    let mut result = vec![];
    let mut i: u128 = 0;

    for _n in 0..8 {
        let (new_i, output) = hash(&md5, i);
        i = new_i;
        println!("Checked {new_i} hashes");
        let c = output[2];

        result.push(c);
    }

    result.iter().fold(String::new(), |mut output, c| {
        let _ = write!(output, "{c:x?}");
        output
    })
}

fn problem2(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.update(input);

    let mut result = ['_'; 8];
    let mut i: u128 = 0;

    loop {
        let (new_i, output) = hash(&md5, i);
        i = new_i;
        println!("Checked {new_i} hashes");

        let result_idx = output[2] as usize;
        if let Some(x) = result.get_mut(result_idx) {
            // don't reassign over an already assigned char
            if *x != '_' {
                continue;
            }

            let c = output[3] >> 4;
            *x = format!("{c:x?}").chars().next().unwrap();

            // this could be part of the outer loop, but that would really slow it all down
            if result.iter().all(|c| *c != '_') {
                break;
            }
        }
    }

    result.iter().collect()
}

#[cfg(test)]
mod test {

    use crate::{problem1, problem2};
    #[test]
    #[ignore = "too slow"]
    fn first() {
        let input = "abc";
        let result = problem1(input);
        assert_eq!(result, "18f47a30")
    }

    #[test]
    #[ignore = "too slow"]
    fn second() {
        let input = "abc";
        let result = problem2(input);
        assert_eq!(result, "05ace8e3")
    }
}
