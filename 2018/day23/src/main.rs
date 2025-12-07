use common::{answer, read_input};
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};

#[cfg(feature = "z3")]
use z3::{ast::Int, Config, Context, Optimize, SatResult};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Vec<Nanobot>;

struct Nanobot {
    position: (i64, i64, i64),
    radius: u64,
}

impl Nanobot {
    fn distance(&self, other: &Nanobot) -> u64 {
        let (sx, sy, sz) = self.position;
        let (ox, oy, oz) = other.position;

        sx.abs_diff(ox) + sy.abs_diff(oy) + sz.abs_diff(oz)
    }

    fn in_range(&self, other: &Nanobot) -> bool {
        self.distance(other) <= self.radius
    }
}

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        map(
            separated_pair(
                delimited(
                    tag("pos=<"),
                    (terminated(i64, tag(",")), terminated(i64, tag(",")), i64),
                    tag(">"),
                ),
                tag(", "),
                preceded(tag("r="), u64),
            ),
            |(position, radius)| Nanobot { position, radius },
        ),
    ).parse(input);

    result.unwrap().1
}

fn problem1(input: &Input) -> usize {
    let strongest = input.iter().max_by_key(|x| x.radius).unwrap();

    input
        .iter()
        .filter(|other| strongest.in_range(other))
        .count()
}

/* This is kind of amazing. I did a binary search that wasn't really working at first, then resorted
 * to cheating with Reddit. A lot of solutions use the Z3 solver to turn this into a SAT problem and then
 * crunch through the problem space. This ran in about 3 min on the input set, which is ...slow...but it works
 *
 * https://cprimozic.net/blog/a-rusty-aoc/#day-23-using-the-z3-smt-solver
 *
 * An alternative would have been an Octree, which looks like what I was initially trying to do with my stupid
 * binary search space algorithm, but I wasn't particularly smart about it.
 *
 * https://www.forrestthewoods.com/blog/solving-advent-of-code-in-under-a-second/
*/
fn problem2(_input: &Input) -> u64 {
    #[cfg(feature = "z3")]
    {
        fn abs_diff<'ctx>(ctx: &'ctx Context, a: &Int<'ctx>, b: &Int<'ctx>) -> Int<'ctx> {
            abs(ctx, &(a - b))
        }

        fn abs<'ctx>(ctx: &'ctx Context, i: &Int<'ctx>) -> Int<'ctx> {
            let zero = Int::from_i64(ctx, 0);
            let negative_one = Int::from_i64(ctx, -1);

            i.gt(&zero).ite(i, &(i * &negative_one))
        }
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let o = Optimize::new(&ctx);

        let zx = Int::new_const(&ctx, "x");
        let zy = Int::new_const(&ctx, "y");
        let zz = Int::new_const(&ctx, "z");

        let mut in_range = Int::from_i64(&ctx, 0);

        for bot in input {
            let (x, y, z) = bot.position;
            println!("Adding constraint for bot at {:?}", bot.position);
            let dist_x = abs_diff(&ctx, &Int::from_i64(&ctx, x), &zx);
            let dist_y = abs_diff(&ctx, &Int::from_i64(&ctx, y), &zy);
            let dist_z = abs_diff(&ctx, &Int::from_i64(&ctx, z), &zz);

            let distance_to_bot = dist_x + dist_y + dist_z;

            let is_in_range_of_bot = distance_to_bot.lt(&Int::from_u64(&ctx, bot.radius + 1));
            in_range += is_in_range_of_bot.ite(&Int::from_u64(&ctx, 1), &Int::from_u64(&ctx, 0));
        }

        let distance_to_origin = &zx + &zy + &zz;

        o.maximize(&in_range);
        o.minimize(&distance_to_origin);

        println!("Optimizing with Z3");
        if SatResult::Sat == o.check(&[]) {
            let model = o.get_model().unwrap();
            let x = model.eval(&zx, true).unwrap().as_i64().unwrap();
            let y = model.eval(&zy, true).unwrap().as_i64().unwrap();
            let z = model.eval(&zz, true).unwrap().as_i64().unwrap();
            let distance = model
                .eval(&distance_to_origin, true)
                .unwrap()
                .as_u64()
                .unwrap();
            println!("The best coordinate is ({x},{y},{z}) at distance {distance}");
            distance
        } else {
            unreachable!("The model was not satisfied!")
        }
    }

    #[cfg(not(feature = "z3"))]
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 7);
    }

    #[test]
    #[cfg(feature = "z3")]
    fn second() {
        let input = include_str!("../test2.txt");
        let input = parse(input);
        let result = crate::problem2(&input);
        assert_eq!(result, 36);
    }
}
