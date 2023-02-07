use nom::{
    bytes::complete::tag, character::complete::u32 as nom_u32, combinator::map,
    multi::separated_list1, IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let mut school = School::new(&input);
    school.simulate(80);
    let first = school.size();
    println!("first answer: {first}");

    let mut school = School::new(&input);
    school.simulate(256);
    let answer = school.size();
    println!("second answer: {answer}");
}

fn parse(input: &str) -> Vec<usize> {
    let result: IResult<&str, Vec<usize>> =
        separated_list1(tag(","), map(nom_u32, |x| x as usize))(input);

    result.unwrap().1
}

#[derive(Clone)]
struct School {
    fish: Vec<i64>,
}

impl School {
    fn new(input: &[usize]) -> School {
        let mut fish = [0i64; 9];
        for f in input {
            fish[*f] += 1;
        }

        School {
            fish: fish.to_vec(),
        }
    }

    fn simulate(&mut self, days: usize) {
        for n in 0..days {
            println!("simulating day {n}");
            let today = n % 9;
            self.fish[(today + 7) % 9] += self.fish[today];
        }
    }

    fn size(&self) -> i64 {
        self.fish.iter().sum()
    }
}
#[test]
fn first() {
    let input = include_str!("../test.txt");
    let input = parse(input);
    let mut school = School::new(&input);
    school.simulate(18);
    assert_eq!(26, school.size());
    school.simulate(62);
    assert_eq!(5934, school.size());
}

#[test]
fn second() {
    let input = include_str!("../test.txt");
    let input = parse(input);
    let mut school = School::new(&input);
    school.simulate(256);
    assert_eq!(26984457539, school.size());
}
