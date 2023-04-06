use nom::{
    bytes::complete::tag, character::complete::i64, combinator::map, multi::separated_list1,
    IResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => unreachable!("Encountered an unimplemented parameter mode: {value}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Halt,
}

impl From<i64> for Opcode {
    fn from(value: i64) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            9 => Self::AdjustRelativeBase,
            99 => Self::Halt,
            _ => unimplemented!("{value} isn't a valid opcode"),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
struct Instruction {
    opcode: Opcode,
    a_mode: ParameterMode,
    b_mode: ParameterMode,
    c_mode: ParameterMode,
}

impl From<i64> for Instruction {
    fn from(value: i64) -> Self {
        Instruction {
            opcode: (value % 100).into(),
            a_mode: ((value / 100) % 10).into(),
            b_mode: ((value / 1000) % 10).into(),
            c_mode: ((value / 10000) % 10).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionResult {
    Halted,
    WaitingForInput,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Intcode {
    pub program: Vec<i64>,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    original_len: usize,
    pc: usize,
    relative_base: i64,
}

impl Intcode {
    pub fn new(input: &[i64]) -> Self {
        // keep track of the original length because we need to terminate off the end of the list
        let original_len = input.len();

        // make sure we have plenty of writeable memory
        let mut program = input.to_vec();
        program.extend(vec![0; 100]);

        Intcode {
            program,
            original_len,
            input: vec![],
            output: vec![],
            pc: 0,
            relative_base: 0,
        }
    }

    pub fn get_location0(&self) -> i64 {
        self.program[0]
    }

    pub fn get_last_output(&self) -> i64 {
        *self.output.last().unwrap()
    }

    fn get_location(&self, location: usize, instruction: &Instruction) -> (i64, ParameterMode) {
        let mode = match location {
            1 => instruction.a_mode,
            2 => instruction.b_mode,
            3 => instruction.c_mode,
            _ => unreachable!("Asked for a parameter that was outside the range"),
        };

        let location = self.pc + location;
        let x = self.program[location];

        let x = match mode {
            ParameterMode::Position => x,
            ParameterMode::Relative => self.relative_base + x,
            ParameterMode::Immediate => x,
        };

        (x, mode)
    }

    fn get_parameter_mut(
        &mut self,
        location: usize,
        instruction: &Instruction,
    ) -> Option<&mut i64> {
        self.grow_if_necessary(location);
        match self.get_location(location, instruction) {
            (_, ParameterMode::Immediate) => {
                unreachable!("Writeable parameters will never be in immediate mode")
            }
            (x, _) => {
                self.grow_if_necessary(x as usize);
                self.program.get_mut(x as usize)
            }
        }
    }

    fn grow_if_necessary(&mut self, location: usize) {
        if location >= self.program.len() {
            let diff = location.abs_diff(self.program.len()) + 1;
            self.program.extend(vec![0; diff]);
        }
    }

    fn get_parameter(&mut self, location: usize, instruction: &Instruction) -> i64 {
        self.grow_if_necessary(location);
        match self.get_location(location, instruction) {
            (x, ParameterMode::Immediate) => x,
            (x, _) => {
                self.grow_if_necessary(x as usize);
                self.program[x as usize]
            }
        }
    }

    fn get_instruction(&self) -> Option<Instruction> {
        // check that we're not off the end of the original program, just in case we somehow didn't halt yet
        (self.pc < self.original_len)
            .then_some(self.pc)
            .and_then(|pc| {
                self.program
                    .get(pc)
                    .map(|raw_opcode| Instruction::from(*raw_opcode))
            })
    }

    fn get_abc(&mut self, instruction: &Instruction) -> Option<(i64, i64, &mut i64)> {
        let a = self.get_parameter(1, instruction);
        let b = self.get_parameter(2, instruction);
        let c = self.get_parameter_mut(3, instruction);

        c.map(|c| (a, b, c))
    }

    fn jump_to(&mut self, location: i64) -> usize {
        self.pc = location as usize;
        // jump never increments the program counter afterward
        0
    }

    pub fn execute(&mut self) -> ExecutionResult {
        while let Some(instruction) = self.get_instruction() {
            let next = match instruction.opcode {
                Opcode::Halt => return ExecutionResult::Halted,
                Opcode::Add => {
                    if let Some((a, b, c)) = self.get_abc(&instruction) {
                        *c = a + b;
                    }

                    4
                }
                Opcode::Multiply => {
                    if let Some((a, b, c)) = self.get_abc(&instruction) {
                        *c = a * b;
                    }

                    4
                }
                Opcode::Input => {
                    if let Some(input) = self.input.pop() {
                        if let Some(result) = self.get_parameter_mut(1, &instruction) {
                            *result = input;
                        }

                        2
                    } else {
                        return ExecutionResult::WaitingForInput;
                    }
                }
                Opcode::Output => {
                    let a = self.get_parameter(1, &instruction);
                    self.output.push(a);

                    2
                }
                Opcode::JumpIfTrue => {
                    let a = self.get_parameter(1, &instruction);
                    let b = self.get_parameter(2, &instruction);

                    if a != 0 {
                        self.jump_to(b)
                    } else {
                        3
                    }
                }
                Opcode::JumpIfFalse => {
                    let a = self.get_parameter(1, &instruction);
                    let b = self.get_parameter(2, &instruction);

                    if a == 0 {
                        self.jump_to(b)
                    } else {
                        3
                    }
                }
                Opcode::LessThan => {
                    if let Some((a, b, c)) = self.get_abc(&instruction) {
                        *c = (a < b).into();
                    }

                    4
                }
                Opcode::Equals => {
                    if let Some((a, b, c)) = self.get_abc(&instruction) {
                        *c = (a == b).into();
                    }

                    4
                }
                Opcode::AdjustRelativeBase => {
                    let a = self.get_parameter(1, &instruction);

                    self.relative_base += a;

                    2
                }
            };

            self.pc += next;
        }

        ExecutionResult::Halted
    }

    pub fn set_noun_verb(&mut self, noun: i64, verb: i64) {
        if let Some(x) = self.program.get_mut(1) {
            *x = noun;
        }
        if let Some(x) = self.program.get_mut(2) {
            *x = verb;
        }
    }

    pub fn parse(s: &str) -> Self {
        let result: IResult<&str, Self> =
            map(separated_list1(tag(","), i64), |x| Intcode::new(&x))(s);
        result.unwrap().1
    }

    /// Useful for just executing a program that takes one input and yields one output
    pub fn execute_simple(&mut self, input: i64) -> i64 {
        self.input.push(input);
        self.execute();
        self.get_last_output()
    }
}

#[allow(dead_code)]
fn run_simple_program(program: &[i64], input: i64) -> i64 {
    let mut p = Intcode::new(program);
    p.execute_simple(input)
}

#[test]
fn instruction_test() {
    let i = Instruction::from(1002);

    assert_eq!(i.opcode, Opcode::Multiply);
    assert_eq!(i.c_mode, ParameterMode::Position);
    assert_eq!(i.b_mode, ParameterMode::Immediate);
    assert_eq!(i.a_mode, ParameterMode::Position);

    let mut p = Intcode::new(&[1002, 4, 3, 4, 33]);
    p.execute();
    assert_eq!(p.program[4], 99);
}

#[test]
fn equal_test() {
    // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    let position_equal = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(run_simple_program(position_equal, 8), 1);
    assert_eq!(run_simple_program(position_equal, 7), 0);

    // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    let immediate_equal = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];
    assert_eq!(run_simple_program(immediate_equal, 8), 1);
    assert_eq!(run_simple_program(immediate_equal, 7), 0);
}

#[test]
fn less_than_test() {
    // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    let position_equal = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(run_simple_program(position_equal, 6), 1);
    assert_eq!(run_simple_program(position_equal, 9), 0);

    // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    let immediate_equal = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];
    assert_eq!(run_simple_program(immediate_equal, 6), 1);
    assert_eq!(run_simple_program(immediate_equal, 9), 0);
}

#[test]
fn jump_test() {
    // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
    let position_jump = &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    assert_eq!(run_simple_program(position_jump, 0), 0);
    assert_eq!(run_simple_program(position_jump, 19), 1);

    let immediate_jump = &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    assert_eq!(run_simple_program(immediate_jump, 0), 0);
    assert_eq!(run_simple_program(immediate_jump, 19), 1);
}

#[test]
fn larger() {
    let program = &[
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];

    assert_eq!(run_simple_program(program, 7), 999);
    assert_eq!(run_simple_program(program, 8), 1000);
    assert_eq!(run_simple_program(program, 9), 1001);
}

#[test]
fn relative_base() {
    let program = &[109, 19, 204, -34, 99];
    let mut p = Intcode::new(program);
    p.relative_base = 2000;
    p.grow_if_necessary(1985);
    p.program[1985] = 100;
    p.execute();
    assert_eq!(p.get_last_output(), 100);
}

#[test]
fn big_number_support() {
    let program = &[104, 1125899906842624i64, 99];
    let mut p = Intcode::new(program);
    p.execute();
    assert_eq!(p.get_last_output(), 1125899906842624i64);
}

#[test]
fn quine() {
    let program = &[
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut p = Intcode::new(program);
    p.execute();
    assert_eq!(p.output, program);
}
