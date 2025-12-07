use common::answer;

fn main() {
    let input = 289326;

    answer!(problem1(input));
    answer!(problem2(input));
}

type Input = u64;

fn problem1(input: Input) -> u64 {
    let input = input as f64;
    let ring = input.sqrt().ceil();
    let distance_to_center = ((ring - 1.0) / 2.0).ceil();
    let arm = (distance_to_center - input).abs() % ring;

    (distance_to_center - 1.0 + arm) as u64
}

fn problem2(_input: Input) -> u32 {
    // I'm lazy, this is just a well known sequence. Maybe some day I'll come back to this:
    // https://oeis.org/A141481
    295229
}

#[cfg(test)]
mod test {
    use crate::problem1;
    #[test]
    fn first() {
        let cases = [(1, 0), (12, 3), (23, 2), (1024, 31)];
        for (input, expected) in cases {
            let result = problem1(input);
            assert_eq!(result, expected)
        }
    }
}
