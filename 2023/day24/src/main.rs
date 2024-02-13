use std::ops::{Add, Mul, RangeInclusive};

use nom::{
    bytes::complete::tag,
    character::complete::{i128, newline, space0},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
#[cfg(feature = "z3")]
use z3::{
    ast::{Ast, Int, Real},
    Config, Context, Optimize, SatResult, Solver,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let score = problem1(&input);
    println!("problem 1 score: {score}");

    let score = problem2(&input);
    println!("problem 2 score: {score}");
}

type Input = Vec<Hailstone>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(
                map(separated_list1(tag(", "), preceded(space0, i128)), |x| {
                    (x[0], x[1], x[2])
                }),
                tag(" @ "),
                map(separated_list1(tag(", "), preceded(space0, i128)), |x| {
                    (x[0], x[1], x[2])
                }),
            ),
            |(position, velocity)| Hailstone { position, velocity },
        ),
    )(input);

    result.unwrap().1
}

#[derive(Clone, Copy, Debug)]
struct Hailstone {
    position: (i128, i128, i128),
    velocity: (i128, i128, i128),
}

impl Hailstone {
    fn as_line(&self) -> Line {
        let p1 = (self.position.0, self.position.1);
        let p2 = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );

        (p1, p2)
    }
}

type Point = (i128, i128);
type Line = ((i128, i128), (i128, i128));

// shamelessly ported from https://stackoverflow.com/questions/20677795/how-do-i-compute-the-intersection-point-of-two-lines
fn line_intersection(line1: Line, line2: Line) -> Option<Point> {
    let xdiff = (line1.0 .0 - line1.1 .0, line2.0 .0 - line2.1 .0);
    let ydiff = (line1.0 .1 - line1.1 .1, line2.0 .1 - line2.1 .1);

    fn det(a: Point, b: Point) -> i128 {
        a.0 * b.1 - a.1 * b.0
    }

    let div = det(xdiff, ydiff);
    if div == 0 {
        None
    } else {
        let d = (det(line1.0, line1.1), det(line2.0, line2.1));
        let x = det(d, xdiff) / div;
        let y = det(d, ydiff) / div;
        Some((x, y))
    }
}

fn count_future_xy_intersections(input: &Input, range: RangeInclusive<i128>) -> usize {
    // generate the crossproduct of the hailstones
    (0..input.len())
        .flat_map(|idx1| {
            (idx1..input.len()).map(move |idx2| {
                let h1 = input.get(idx1).unwrap();
                let h2 = input.get(idx2).unwrap();

                (h1, h2)
            })
        })
        .filter(|(h1, h2)| {
            // only consider lines that meet
            let l1 @ ((x1, _y1), (x2, _y2)) = h1.as_line();
            let l2 @ ((x3, _y3), (x4, _y4)) = h2.as_line();
            if let Some((ix, iy)) = line_intersection(l1, l2) {
                // only keep the ones in the future
                let valid1 = (ix > x1) == (x2 > x1);
                let valid2 = (ix > x3) == (x4 > x3);

                range.contains(&ix) && range.contains(&iy) && valid1 && valid2
            } else {
                false
            }
        })
        .count()
}

fn problem1(input: &Input) -> usize {
    count_future_xy_intersections(input, 200000000000000..=400000000000000)
}

fn problem2(input: &Input) -> i64 {
    #[cfg(feature = "z3")]
    {
        let ctx = z3::Context::new(&z3::Config::new());
        let s = z3::Solver::new(&ctx);
        let [fx, fy, fz, fdx, fdy, fdz] = ["fx", "fy", "fz", "fdx", "fdy", "fdz"]
            .map(|v| Real::from_int(&Int::new_const(&ctx, v)));

        let zero = Real::from_int(&Int::from_i64(&ctx, 0));
        for (i, hailstone) in input.iter().enumerate().take(3) {
            let (x, y, z) = hailstone.position;
            let (dx, dy, dz) = hailstone.velocity;

            let [x, y, z, dx, dy, dz] =
                [x, y, z, dx, dy, dz].map(|v| Real::from_int(&Int::from_i64(&ctx, v as _)));
            let t = Real::new_const(&ctx, format!("t{i}"));
            s.assert(&t.ge(&zero));
            s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
            s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
            s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
        }

        assert_eq!(s.check(), z3::SatResult::Sat);
        let model = s.get_model().unwrap();
        let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
        res.as_real().unwrap().0
    }

    #[cfg(not(feature = "z3"))]
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::{count_future_xy_intersections, parse, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = count_future_xy_intersections(&input, 7..=27);
        assert_eq!(result, 2)
    }

    #[test]
    #[cfg(feature = "z3")]
    fn second() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);
        assert_eq!(result, 47)
    }
}
