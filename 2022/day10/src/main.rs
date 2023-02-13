use std::vec;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::map,
    multi::separated_list1, sequence::preceded, IResult,
};

fn main() {
    let input = include_str!("../input.txt");
    let input = parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 score:\n{answer}");
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i32),
}

type Input = Vec<Instruction>;

fn parse(input: &str) -> Input {
    let result: IResult<&str, Input> = separated_list1(
        newline,
        alt((
            map(tag("noop"), |_| Instruction::NoOp),
            map(preceded(tag("addx "), nom::character::complete::i32), |x| {
                Instruction::AddX(x)
            }),
        )),
    )(input);

    result.unwrap().1
}

const INTERESTING: [u32; 6] = [20, 60, 100, 140, 180, 220];
fn problem1(lines: &Input) -> i32 {
    let mut signals = vec![];

    let mut cpu = Cpu::new(lines);
    cpu.execute(|CycleResult { cycle, register_x }| {
        if INTERESTING.contains(&cycle) {
            let signal_strength = (cycle as i32) * register_x;
            signals.push(signal_strength);
        }
    });

    signals.iter().sum()
}

#[derive(Debug)]
struct Crt {
    pixels: [bool; 240],
}

impl Crt {
    fn draw(&mut self, CycleResult { cycle, register_x }: CycleResult) {
        let current = (cycle - 1) % 40;

        let on = (register_x - 1..=register_x + 1).any(|x| x == (current as i32));
        self.pixels[(cycle - 1) as usize] = on;
    }

    fn get_message(&self) -> String {
        let v: Vec<String> = self
            .pixels
            .chunks(40)
            .map(|x| {
                x.iter()
                    .map(|x| match x {
                        true => "#",
                        false => " ",
                    })
                    .collect()
            })
            .collect();

        v.join("\n")
    }
}

fn problem2(lines: &Input) -> String {
    let mut cpu = Cpu::new(lines);
    let mut crt = Crt {
        pixels: [false; 240],
    };

    cpu.execute(|result| {
        crt.draw(result);
    });

    crt.get_message()
}

struct Cpu<'a> {
    cycle: u32,
    register_x: i32,
    instructions: &'a [Instruction],
}

struct CycleResult {
    cycle: u32,
    register_x: i32,
}

impl<'a> Cpu<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Cpu {
            register_x: 1,
            cycle: 0,
            instructions,
        }
    }

    fn execute(&mut self, mut f: impl FnMut(CycleResult)) {
        for i in self.instructions {
            match i {
                Instruction::AddX(v) => {
                    self.cycle += 1;
                    f(CycleResult {
                        cycle: self.cycle,
                        register_x: self.register_x,
                    });

                    self.cycle += 1;
                    f(CycleResult {
                        cycle: self.cycle,
                        register_x: self.register_x,
                    });

                    self.register_x += v;
                }
                Instruction::NoOp => {
                    self.cycle += 1;
                    f(CycleResult {
                        cycle: self.cycle,
                        register_x: self.register_x,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{parse, problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem1(&input);
        assert_eq!(result, 13140)
    }

    #[test]
    fn second() {
        const EXPECTED: &str = "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     ";

        let input = include_str!("../test.txt");
        let input = parse(input);
        let result = problem2(&input);

        assert_eq!(result, EXPECTED)
    }
}
