use std::{cmp::Reverse, collections::BinaryHeap, time::Instant};

use nom::{
    bytes::complete::tag,
    character::complete::{char, newline, u64},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let i = Instant::now();
    let score = problem1(&input);
    let d = i.elapsed();
    println!("problem 1 score: {score} in {d:?}");

    let i = Instant::now();
    let score = problem2(&input);
    let d = i.elapsed();
    println!("problem 2 score: {score} in {d:?}");
}

type Input = ((u64, u64, u64), Vec<u64>);

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_pair(
        tuple((
            delimited(tag("Register A: "), u64, newline),
            delimited(tag("Register B: "), u64, newline),
            delimited(tag("Register C: "), u64, newline),
        )),
        newline,
        preceded(tag("Program: "), separated_list1(char(','), u64)),
    )(input);

    result.unwrap().1
}

fn execute(registers: (u64, u64, u64), instructions: &[u64]) -> Vec<u64> {
    let (mut a, mut b, mut c) = registers;
    let mut pc = 0;
    let mut out = vec![];

    while let Some((instruction, operand)) = instructions.get(pc).zip(instructions.get(pc + 1)) {
        let combo: Option<u64> = match *operand {
            x if x < 4 => Some(x),
            4 => Some(a),
            5 => Some(b),
            6 => Some(c),
            _ => None,
        };
        match instruction {
            0 => {
                // adv
                let numerator = a;
                let denominator = 2_u64.pow(combo.unwrap() as u32);

                let result = numerator / denominator;
                a = result;
            }
            1 => {
                //bxl
                let result = b ^ operand;
                b = result;
            }
            2 => {
                // bst
                let result = combo.unwrap() % 8;
                b = result;
            }
            3 => {
                if a != 0 {
                    {
                        //jnz
                        pc = *operand as usize;
                        continue;
                    }
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                let result = combo.unwrap() % 8;
                out.push(result);
            }
            6 => {
                let numerator = a;
                let denominator = 2_u64.pow(combo.unwrap() as u32);

                let result = numerator / denominator;
                b = result;
            }
            7 => {
                let numerator = a;
                let denominator = 2_u64.pow(combo.unwrap() as u32);

                let result = numerator / denominator;
                c = result;
            }

            x => todo!("{}", x),
        }

        pc += 2;
    }

    out
}

fn problem1(input: &Input) -> String {
    let (registers, program) = input;
    let out = execute(*registers, program);

    let s: Vec<String> = out.iter().map(|x| x.to_string()).collect();
    s.join(",").to_string()
}

fn problem2(input: &Input) -> u64 {
    let (_, program) = input.clone();
    // use a min-heap so we're guaranteed to find the smallest input
    let mut v = BinaryHeap::new();
    v.push(Reverse((0, program.len() - 1)));

    while let Some(Reverse((a, len))) = v.pop() {
        // shift the last three bits over
        let a = a << 3;
        for a in a..a + 8 {
            let out = execute((a, 0, 0), &program);
            // if we found it all, return
            if out == program {
                return a;
            }

            // if this number matches the end of the program, keep going
            if out[..] == program[len..] {
                v.push(Reverse((a, len - 1)));
            }
        }
    }

    unreachable!("Somehow we failed to find a solution")
}

#[cfg(test)]
mod test {
    use crate::{parse, problem1, problem2};

    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string())
    }

    #[test]
    fn second() {
        let input = ((2024, 0, 0), vec![0, 3, 5, 4, 3, 0]);
        let result = problem2(&input);
        assert_eq!(result, 117440);
    }
}
