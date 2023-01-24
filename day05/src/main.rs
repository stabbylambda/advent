use common::get_raw_input;
use crypto::{digest::Digest, md5::Md5};

fn main() {
    let input = get_raw_input();

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

fn hash(md5: &Md5, start: u128) -> (u128, [u8; 16]) {
    for i in start.. {
        let mut hasher = md5.clone();
        hasher.input_str(&i.to_string());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        let valid = (output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32) == 0;
        if valid {
            return (i + 1, output.clone());
        }
        hasher.reset();
    }

    unreachable!()
}

fn problem1(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.input_str(input);

    let mut result = vec![];
    let mut i: u128 = 0;

    for _n in 0..8 {
        let (new_i, output) = hash(&md5, i);
        i = new_i;
        println!("Checked {new_i} hashes");
        let c = output[2] as u8;

        result.push(c);
    }

    result.iter().map(|c| format!("{c:x?}")).collect()
}

fn problem2(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.input_str(input);

    let mut result = vec!['_'; 8];
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
    fn first() {
        let input = "abc";
        let result = problem1(&input);
        assert_eq!(result, "18f47a30")
    }

    #[test]
    fn second() {
        let input = "abc";
        let result = problem2(&input);
        assert_eq!(result, "05ace8e3")
    }
}
