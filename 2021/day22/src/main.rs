use counter::Counter;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Vec<Step>;

fn parse(input: &str) -> Input {
    let range = |s| separated_pair(i64, tag(".."), i64)(s);
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            tuple((
                alt((map(tag("on "), |_| true), map(tag("off "), |_| false))),
                delimited(tag("x="), range, tag(",")),
                delimited(tag("y="), range, tag(",")),
                preceded(tag("z="), range),
            )),
            |(on, x, y, z)| Step {
                on,
                cube: Cube { x, y, z },
            },
        ),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug)]
struct Step {
    on: bool,
    cube: Cube,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cube {
    fn intersect(&self, other: &Cube) -> Option<Cube> {
        let c = Cube {
            x: (self.x.0.max(other.x.0), self.x.1.min(other.x.1)),
            y: (self.y.0.max(other.y.0), self.y.1.min(other.y.1)),
            z: (self.z.0.max(other.z.0), self.z.1.min(other.z.1)),
        };

        (c.x.0 <= c.x.1 && c.y.0 <= c.y.1 && c.z.0 <= c.z.1).then_some(c)
    }

    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }
}

// this is just an application of https://en.wikipedia.org/wiki/Inclusion–exclusion_principle
fn count_cubes(steps: &[Step]) -> i64 {
    let cubes = steps.iter().fold(Counter::new(), |cubes, step| {
        let mut update: Counter<Cube, i64> = Counter::new();

        // first include this cube
        if step.on {
            update.insert(step.cube, 1);
        }

        let intersections = cubes
            .iter()
            .filter_map(|(c, sign)| step.cube.intersect(c).map(|i| (i, *sign)));

        // now exclude it from the counts of all other cubes that it's intersecting
        for (i, sign) in intersections {
            let x = update.entry(i).or_insert(0);
            *x -= sign;
        }

        cubes + update
    });

    cubes.iter().map(|(cube, sign)| cube.volume() * sign).sum()
}

fn problem1(input: &Input) -> i64 {
    let init_region = Cube {
        x: (-50, 50),
        y: (-50, 50),
        z: (-50, 50),
    };

    let in_init_region: Vec<Step> = input
        .iter()
        .filter(|x| x.cube.intersect(&init_region).is_some())
        .copied()
        .collect();

    count_cubes(&in_init_region)
}

fn problem2(input: &Input) -> i64 {
    count_cubes(input)
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 590784)
    }

    #[test]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 39769202357779)
    }
}
