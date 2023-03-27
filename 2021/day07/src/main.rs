use common::math::div_ceil;
use nom::{
    bytes::complete::tag, character::complete::i32 as nom_i32, multi::separated_list1, IResult,
};
fn main() {
    let input = include_str!("../input.txt");
    let crabs = parse(input);
    let fleet = CrabFleet {
        crabs: crabs.clone(),
    };
    let fuel = fleet.align1();
    println!("first answer {fuel}");

    let fleet = CrabFleet { crabs };
    let fuel = fleet.align2();
    println!("second answer {fuel}")
}
fn parse(input: &str) -> Vec<i32> {
    let result: IResult<&str, Vec<i32>> = separated_list1(tag(","), nom_i32)(input);

    result.unwrap().1
}

struct CrabFleet {
    crabs: Vec<i32>,
}

impl CrabFleet {
    fn align1(&self) -> i32 {
        let position = self.calculate_median();
        let mut total = 0;
        for c in &self.crabs {
            total += (c - position).abs()
        }
        total
    }
    fn align2(&self) -> i32 {
        // you need to calculate the mean with both rounding up and down...which sucks
        // this only showed up in the real data set, not in the test set...bummer
        let (mean1, mean2) = self.calculate_means();
        let (mut total1, mut total2) = (0, 0);

        fn fuel(n: i32) -> i32 {
            (n * (n + 1)) / 2
        }

        for c in &self.crabs {
            total1 += fuel((c - mean1).abs());
            total2 += fuel((c - mean2).abs());
        }

        total1.min(total2)
    }

    fn calculate_means(&self) -> (i32, i32) {
        let count: i32 = self.crabs.len().try_into().unwrap();
        let sum = self.crabs.iter().sum();

        (div_ceil(sum, count), sum / count)
    }

    fn calculate_median(&self) -> i32 {
        let mut values = self.crabs.clone();
        values.sort();
        let length = values.len();
        if length % 2 == 0 {
            let mid1 = values.get(length / 2).unwrap();
            let mid2 = values.get((length / 2) - 1).unwrap();

            (*mid1 + *mid2) / 2
        } else {
            let mid1 = values.get(length / 2).unwrap();

            *mid1
        }
    }
}

#[test]
fn first() {
    let input = include_str!("../test.txt");
    let crabs = parse(input);
    let crabs = CrabFleet { crabs };
    let fuel = crabs.align1();
    assert_eq!(37, fuel);
}

#[test]
fn second() {
    let input = include_str!("../test.txt");
    let crabs = parse(input);
    let crabs = CrabFleet { crabs };
    let fuel = crabs.align2();
    assert_eq!(168, fuel);
}
