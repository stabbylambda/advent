use std::collections::HashSet;
fn main() {
    let input = common::read_input!();
    let displays = input.lines().map(get_display).collect();
    let displays = Displays { displays };
    let total = displays.count_easy_numbers();
    println!("first answer {total}");
    let total = displays.decode();
    println!("second answer {total}");
}

fn get_display(s: &str) -> Display {
    let [first, last] = s.split(" | ").collect::<Vec<&str>>()[..] else {
        panic!("Not a valid input file");
    };
    let input: Vec<String> = first.split_whitespace().map(|s| s.to_owned()).collect();
    let output: Vec<String> = last.split_whitespace().map(|s| s.to_owned()).collect();
    let digits = [&input[..], &output[..]].concat();

    Display { output, digits }
}

struct Displays {
    displays: Vec<Display>,
}
struct Display {
    output: Vec<String>,
    digits: Vec<String>,
}

impl Display {
    fn digits_of_length(&self, n: usize) -> HashSet<char> {
        self.digits
            .iter()
            .filter(|x| x.len() == n)
            .map(|s| HashSet::from_iter(s.chars()))
            .next()
            .unwrap()
    }

    fn all_digits_of_length(&self, n: usize) -> Vec<HashSet<char>> {
        self.digits
            .iter()
            .filter(|x| x.len() == n)
            .map(|s| HashSet::from_iter(s.chars()))
            .collect()
    }

    fn decode(&self) -> i32 {
        let one_digits = self.digits_of_length(2);
        let seven_digits = self.digits_of_length(3);
        let four_digits = self.digits_of_length(4);
        let eight_digits = self.digits_of_length(7);

        let nine_candidates = self.all_digits_of_length(6); // 9, 0, and 6

        // 7 is 1 plus a top bar
        let top_light = *seven_digits.difference(&one_digits).next().unwrap();

        // nine is 4 plus the top light
        let mut mask: HashSet<_> = HashSet::from_iter(four_digits.clone());
        mask.insert(top_light);

        // find all the nine candidates and compare them against the mask
        let nine_digits = nine_candidates
            .iter().find(|candidate| candidate.is_superset(&mask))
            .unwrap();

        // nine minus the 4 plus the top light mask gives us the bottom light
        let bottom_light = nine_digits.difference(&mask).next().unwrap();
        let bottom_left_light = eight_digits.difference(nine_digits).next().unwrap();

        let zero_digits = nine_candidates
            .iter()
            // remove the nine since we know that now
            .filter(|candidate| *candidate != nine_digits).find(|candidate| candidate.is_superset(&one_digits))
            .unwrap();

        let middle_light = eight_digits.difference(zero_digits).next().unwrap();

        let six_digits = nine_candidates
            .iter().find(|candidate| *candidate != nine_digits && *candidate != zero_digits)
            .unwrap();

        // the six contains the bottom of the one, but not the top
        let top_right_light = *one_digits.difference(six_digits).next().unwrap();
        let mask: HashSet<char> = HashSet::from_iter(vec![top_right_light]);

        let bottom_right_light = one_digits.difference(&mask).next().unwrap();
        let mask = HashSet::from_iter(vec![
            top_light,
            top_right_light,
            *middle_light,
            *bottom_left_light,
            *bottom_right_light,
            *bottom_light,
        ]);

        let top_left_light = eight_digits.difference(&mask).next().unwrap();

        print!(
            r#"
 {0} {0} {0} {0} 
{1}       {2}
{1}       {2}
 {3} {3} {3} {3} 
{4}       {5}
{4}       {5}
 {6} {6} {6} {6} 
        "#,
            top_light,
            top_left_light,
            top_right_light,
            middle_light,
            bottom_left_light,
            bottom_right_light,
            bottom_light
        );

        // already have the rest, just need to make these
        let two_digits = HashSet::from_iter(vec![
            top_light,
            top_right_light,
            *middle_light,
            *bottom_left_light,
            *bottom_light,
        ]);
        let three_digits = HashSet::from_iter(vec![
            top_light,
            top_right_light,
            *middle_light,
            *bottom_right_light,
            *bottom_light,
        ]);
        let five_digits = HashSet::from_iter(vec![
            top_light,
            *top_left_light,
            *middle_light,
            *bottom_right_light,
            *bottom_light,
        ]);

        let mut s = String::new();
        for digit in &self.output {
            let charset: HashSet<char> = HashSet::from_iter(digit.chars());
            let digit = if charset == *zero_digits {
                0
            } else if charset == one_digits {
                1
            } else if charset == two_digits {
                2
            } else if charset == three_digits {
                3
            } else if charset == four_digits {
                4
            } else if charset == five_digits {
                5
            } else if charset == *six_digits {
                6
            } else if charset == seven_digits {
                7
            } else if charset == eight_digits {
                8
            } else {
                9
            };
            s = format!("{s}{digit}");
        }
        s.parse().unwrap()
    }

    fn count_easy_numbers(&self) -> i32 {
        let mut total = 0;
        for digit in &self.output {
            let length = digit.len();
            if length == 2 || length == 4 || length == 3 || length == 7 {
                total += 1;
            }
        }
        total
    }
}

impl Displays {
    fn decode(&self) -> i32 {
        let mut total = 0;
        for display in &self.displays {
            total += display.decode();
        }
        total
    }
    fn count_easy_numbers(&self) -> i32 {
        let mut total = 0;
        for display in &self.displays {
            total += display.count_easy_numbers();
        }
        total
    }
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let displays = input.lines().map(get_display).collect();
    let displays = Displays { displays };
    let total = displays.count_easy_numbers();

    assert_eq!(total, 26)
}

#[test]
fn middle() {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let display = get_display(input);
    let result = display.decode();
    assert_eq!(result, 5353)
}

#[test]
fn second() {
    let input = include_str!("../test.txt");
    let displays = input.lines().map(get_display).collect();
    let displays = Displays { displays };
    let total = displays.decode();

    assert_eq!(total, 61229)
}
