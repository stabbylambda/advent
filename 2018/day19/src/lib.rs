pub use advent_2018_16::{Instruction, Opcode};
use common::nom::usize;
use common::program::parsing::instruction3;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

pub struct ElfCode {
    pub bound: usize,
    pub program: Vec<Instruction<Opcode>>,
}

impl ElfCode {
    pub fn parse(input: &str) -> Self {
        let opcode_instruction = |name: &'static str, opcode: Opcode| {
            instruction3(name, usize, usize, usize, Instruction::new(opcode))
        };

        let result: IResult<&str, ElfCode> = map(
            separated_pair(
                preceded(tag("#ip "), usize),
                newline,
                separated_list1(
                    newline,
                    alt((
                        opcode_instruction("addr", Opcode::Addr),
                        opcode_instruction("addi", Opcode::Addi),
                        opcode_instruction("mulr", Opcode::Mulr),
                        opcode_instruction("muli", Opcode::Muli),
                        opcode_instruction("banr", Opcode::Banr),
                        opcode_instruction("bani", Opcode::Bani),
                        opcode_instruction("borr", Opcode::Borr),
                        opcode_instruction("bori", Opcode::Bori),
                        opcode_instruction("setr", Opcode::Setr),
                        opcode_instruction("seti", Opcode::Seti),
                        opcode_instruction("gtir", Opcode::Gtir),
                        opcode_instruction("gtri", Opcode::Gtri),
                        opcode_instruction("gtrr", Opcode::Gtrr),
                        opcode_instruction("eqir", Opcode::Eqir),
                        opcode_instruction("eqri", Opcode::Eqri),
                        opcode_instruction("eqrr", Opcode::Eqrr),
                    )),
                ),
            ),
            |(bound, program)| ElfCode { bound, program },
        ).parse(input);

        result.unwrap().1
    }
}
