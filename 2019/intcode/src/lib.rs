use nom::{
    bytes::complete::tag, character::complete::i64, combinator::map, multi::separated_list1,
    IResult,
};

#[derive(Clone, Debug)]
pub struct Intcode {
    pub program: Vec<i64>,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    pc: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from(x: i64) -> Self {
        match x {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unreachable!("Encountered an unimplemented parameter mode: {x}"),
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
    Halt,
}

impl Opcode {
    fn new(x: i64) -> Self {
        match x {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            _ => unimplemented!("{x} isn't a valid opcode"),
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

impl Instruction {
    fn new(x: i64) -> Self {
        let opcode = Opcode::new(x % 100);
        let a_mode = ParameterMode::from((x / 100) % 10);
        let b_mode = ParameterMode::from((x / 1000) % 10);
        let c_mode = ParameterMode::from((x / 10000) % 10);

        Instruction {
            opcode,
            a_mode,
            b_mode,
            c_mode,
        }
    }
}

#[test]
fn instruction_test() {
    let i = Instruction::new(1002);

    assert_eq!(i.opcode, Opcode::Multiply);
    assert_eq!(i.c_mode, ParameterMode::Position);
    assert_eq!(i.b_mode, ParameterMode::Immediate);
    assert_eq!(i.a_mode, ParameterMode::Position);

    let mut p = Intcode::new(&[1002, 4, 3, 4, 33]);
    p.execute();
    assert_eq!(p.program[4], 99);
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionResult {
    Halted,
    WaitingForInput,
}

impl Intcode {
    pub fn new(input: &[i64]) -> Self {
        Intcode {
            program: input.to_vec(),
            input: vec![],
            output: vec![],
            pc: 0,
        }
    }

    pub fn get_location0(&self) -> i64 {
        self.program[0]
    }

    pub fn get_last_output(&self) -> i64 {
        *self.output.last().unwrap()
    }

    fn get_write_location(&mut self, location: usize) -> Option<&mut i64> {
        let x = self.program[location];
        self.program.get_mut(x as usize)
    }

    fn get_parameter(&self, location: usize, mode: ParameterMode) -> i64 {
        let x = self.program[location];

        match mode {
            ParameterMode::Position => self.program[x as usize],
            ParameterMode::Immediate => x,
        }
    }

    fn get_instruction(&self) -> Option<Instruction> {
        self.program
            .get(self.pc)
            .map(|raw_opcode| Instruction::new(*raw_opcode))
    }

    fn get_a(&self, instruction: &Instruction) -> i64 {
        self.get_parameter(self.pc + 1, instruction.a_mode)
    }

    fn get_ab(&self, instruction: &Instruction) -> (i64, i64) {
        let a = self.get_a(instruction);
        let b = self.get_parameter(self.pc + 2, instruction.b_mode);
        (a, b)
    }

    fn get_abc(&mut self, instruction: &Instruction) -> Option<(i64, i64, &mut i64)> {
        let (a, b) = self.get_ab(instruction);
        let c = self.get_write_location(self.pc + 3);

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
                Opcode::Input => match self.input.pop() {
                    Some(input) => {
                        if let Some(result) = self.get_write_location(self.pc + 1) {
                            *result = input;
                        }

                        2
                    }
                    // if we didn't have any input waiting, we need to terminate here
                    None => return ExecutionResult::WaitingForInput,
                },
                Opcode::Output => {
                    let a = self.get_a(&instruction);
                    self.output.push(a);

                    2
                }
                Opcode::JumpIfTrue => {
                    let (a, b) = self.get_ab(&instruction);

                    if a != 0 {
                        self.jump_to(b)
                    } else {
                        3
                    }
                }
                Opcode::JumpIfFalse => {
                    let (a, b) = self.get_ab(&instruction);

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
}

#[allow(dead_code)]
fn run_simple_program(program: &[i64], input: i64) -> i64 {
    let mut p = Intcode::new(program);
    p.input.push(input);
    p.execute();
    p.get_last_output()
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
