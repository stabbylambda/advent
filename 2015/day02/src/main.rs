use nom::{
    character::complete::{char, newline, u32 as nom_u32},
    combinator::map,
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Box>;

struct Box {
    l: u32,
    w: u32,
    h: u32,
}

impl Box {
    fn paper_needed(&self) -> u32 {
        let s1 = self.l * self.w;
        let s2 = self.w * self.h;
        let s3 = self.h * self.l;
        let surface_area = 2 * s1 + 2 * s2 + 2 * s3;
        let extra = s1.min(s2).min(s3);

        surface_area + extra
    }

    fn ribbon_needed(&self) -> u32 {
        let ribbon = 2 * self.l + 2 * self.w;
        let bow = self.l * self.w * self.h;
        ribbon + bow
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(separated_list1(char('x'), nom_u32), |mut v| {
            v.sort();
            Box {
                l: v[0],
                w: v[1],
                h: v[2],
            }
        }),
    )(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> u32 {
    input.iter().map(|b| b.paper_needed()).sum()
}

fn problem2(input: &Input) -> u32 {
    input.iter().map(|b| b.ribbon_needed()).sum()
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let tests = [("2x3x4", 58), ("1x1x10", 43)];

        for (input, expected) in tests {
            assert_eq!(problem1(&parse(input)), expected)
        }
    }

    #[test]
    fn second() {
        let tests = [("2x3x4", 34), ("1x1x10", 14)];
        for (input, expected) in tests {
            assert_eq!(problem2(&parse(input)), expected)
        }
    }
}
