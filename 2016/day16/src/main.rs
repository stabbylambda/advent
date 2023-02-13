fn main() {
    let input = Disc {
        length: 272,
        initial: "01000100010010111",
    };

    let answer = problem(&input);
    println!("problem 1 answer: {answer}");

    let input = Disc {
        length: 35651584,
        initial: "01000100010010111",
    };

    let answer = problem(&input);
    println!("problem 2 answer: {answer}");
}

type Input<'a> = Disc<'a>;
struct Disc<'a> {
    length: usize,
    initial: &'a str,
}

/* This is a garbage solution that is slow for problem 2. The better solution is
 * https://www.reddit.com/r/adventofcode/comments/5ititq/2016_day_16_c_how_to_tame_your_dragon_in_under_a/
 */
fn problem(input: &Input) -> String {
    let mut data = String::with_capacity(input.length);
    data.push_str(input.initial);

    // generate the string
    while data.len() < input.length {
        let b: String = data
            .chars()
            .rev()
            // this could probably be done using bitwise not, but our final doesn't fit in a u128
            // let's see if we can cheat this way
            .map(|c| match c {
                '0' => '1',
                '1' => '0',
                _ => unreachable!(),
            })
            .collect();

        data.push_str(&format!("0{b}"));
    }

    // trim down to the amount we need to fill
    let mut v: Vec<char> = data.chars().take(input.length).collect();
    while v.len() % 2 == 0 {
        v = v
            .chunks(2)
            .map(|x| match (x[0], x[1]) {
                ('0', '0') | ('1', '1') => '1',
                _ => '0',
            })
            .collect();
    }

    v.iter().collect()
}
#[cfg(test)]
mod test {

    use crate::{problem, Disc};
    #[test]
    fn first() {
        let input = Disc {
            length: 12,
            initial: "110010110100",
        };

        assert_eq!(problem(&input), "100");

        let input = Disc {
            length: 20,
            initial: "10000",
        };

        assert_eq!(problem(&input), "01100");
    }
}
