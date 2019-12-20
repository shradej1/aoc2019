/// Implements an Intcode computer
use std::convert::TryFrom;
use std::convert::TryInto;

pub type Result<T> = std::result::Result<T, String>;
pub type MemContent = i32;
pub type Addr = usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ParameterMode {
    /// Parameters are interpreted as a position.  If the parameter is 50, its value is the value
    /// stored at address 50 in memory.
    PositionMode,

    /// Parameters are interpreted as values.  If the parameter is 50, the value is simply 50.
    ImmediateMode,
}

impl ParameterMode {
    fn parse(&self, prog: &Vec<MemContent>, loc: Addr) -> MemContent {
        match self {
            ParameterMode::PositionMode => prog[usize::try_from(prog[loc]).unwrap()],
            ParameterMode::ImmediateMode => prog[loc],
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
}

impl TryFrom<MemContent> for OpCode {
    type Error = String;
    fn try_from(u: MemContent) -> Result<OpCode> {
        match u {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Multiply),
            3 => Ok(OpCode::Input),
            4 => Ok(OpCode::Output),
            99 => Ok(OpCode::Halt),
            _ => Err(format!("Unexpected opcode: {}", u)),
        }
    }
}

impl TryFrom<MemContent> for ParameterMode {
    type Error = String;
    fn try_from(u: MemContent) -> Result<ParameterMode> {
        match u {
            0 => Ok(ParameterMode::PositionMode),
            1 => Ok(ParameterMode::ImmediateMode),
            _ => Err(format!("Unexepcted parameter mode: {}", u)),
        }
    }
}

pub struct Input {
    value: MemContent,
}

impl Input {
    fn new(value: MemContent) -> Self {
        Input { value }
    }
}

impl Input {
    fn read(&mut self) -> MemContent {
        self.value
    }
}

pub struct Output {
    value: MemContent,
}

impl Output {
    fn new() -> Self {
        Output { value: 0 }
    }
}

impl Output {
    fn write(&mut self, value: MemContent) {
        self.value = value;
    }
}

pub struct IntCodeProgramExecutor<T> {
    program: T,
    noun: MemContent,
    verb: MemContent,
    input: Option<Input>,
    output: Option<Output>,
}

impl From<Vec<MemContent>> for IntCodeProgramExecutor<Vec<MemContent>> {
    fn from(program: Vec<MemContent>) -> Self {
        let noun = program[1];
        let verb = program[2];
        IntCodeProgramExecutor {
            program,
            noun,
            verb,
            input: None,
            output: None,
        }
    }
}

impl<'a> From<&'a mut Vec<MemContent>> for IntCodeProgramExecutor<&'a mut Vec<MemContent>> {
    fn from(program: &'a mut Vec<MemContent>) -> Self {
        let noun = program[1];
        let verb = program[2];
        IntCodeProgramExecutor {
            program,
            noun,
            verb,
            input: None,
            output: None,
        }
    }
}

impl IntCodeProgramExecutor<&mut Vec<MemContent>> {
    pub fn input(mut self, input: Input) -> Self {
        self.input = Some(input);
        self
    }

    pub fn output(mut self, output: Output) -> Self {
        self.output = Some(output);
        self
    }

    pub fn execute(&mut self) -> Result<MemContent> {
        let prog = &mut self.program;
        let mut instr_ptr = 0;
        loop {
            /// The opcode is a two-digit number based only on the ones and tens digit of the value
            let opcode = OpCode::try_from(prog[instr_ptr] % 100)?;
            match opcode {
                OpCode::Add => {
                    let param_mode = ParameterMode::try_from(prog[instr_ptr] / 100 % 10)?;
                    let a1 = param_mode.parse(prog, instr_ptr + 1);

                    let param_mode = ParameterMode::try_from(prog[instr_ptr] / 1000 % 10)?;
                    let a2 = param_mode.parse(prog, instr_ptr + 2);

                    let param_mode = ParameterMode::try_from(prog[instr_ptr] / 10000 % 10)?;
                    assert!(param_mode != ParameterMode::ImmediateMode);

                    let dest: Addr = prog[instr_ptr + 3].try_into().unwrap();
                    prog[dest] = a1 + a2;
                    instr_ptr += 4;
                }
                OpCode::Multiply => {
                    let param_mode = ParameterMode::try_from(prog[instr_ptr] / 100 % 10)?;
                    let a1 = param_mode.parse(prog, instr_ptr + 1);

                    let param_mode = ParameterMode::try_from(prog[instr_ptr] / 1000 % 10)?;
                    let a2 = param_mode.parse(prog, instr_ptr + 2);

                    let param_mode = ParameterMode::try_from(prog[instr_ptr] / 10000 % 10)?;
                    assert!(param_mode != ParameterMode::ImmediateMode);

                    let dest: Addr = prog[instr_ptr + 3].try_into().unwrap();
                    prog[dest] = a1 * a2;
                    instr_ptr += 4;
                }
                OpCode::Input => {
                    let store_addr: Addr = prog[instr_ptr + 1].try_into().unwrap();
                    let input = self
                        .input
                        .as_mut()
                        .expect("Input opcode invalid with no input")
                        .read();
                    prog[store_addr] = input;
                    instr_ptr += 2;
                }
                OpCode::Output => {
                    let param_mode = ParameterMode::try_from(prog[instr_ptr] / 100 % 10)?;
                    let output_value = param_mode.parse(prog, instr_ptr + 1);
                    self.output
                        .as_mut()
                        .expect("Output opcode invalid with no output")
                        .write(output_value);
                    instr_ptr += 2;
                }
                OpCode::Halt => break,
            }
        }
        Ok(self.program[0])
    }
}

pub trait IntCodeProgram {
    fn execute(&mut self) -> Result<MemContent>;
    fn execute_with_args(&mut self, arg1: MemContent, arg2: MemContent) -> Result<MemContent>;
    fn output(&self) -> MemContent;
    fn search_for_output(&self, target_output: MemContent) -> Result<(MemContent, MemContent)>;
}

impl IntCodeProgram for Vec<MemContent> {
    fn execute(&mut self) -> Result<MemContent> {
        IntCodeProgramExecutor::from(self).execute()
    }

    fn execute_with_args(&mut self, noun: MemContent, verb: MemContent) -> Result<MemContent> {
        self[1] = noun;
        self[2] = verb;
        self.execute()
    }

    fn output(&self) -> MemContent {
        self[0]
    }

    /// Returns the noun and verb that produce the given input
    ///
    /// Note that this is immutable since the original program state must be restored upon each
    /// execution.
    fn search_for_output(&self, target_output: MemContent) -> Result<(MemContent, MemContent)> {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut prog = self.clone();
                if let Ok(output) = prog.execute_with_args(noun, verb) {
                    if output == target_output {
                        return Ok((noun, verb));
                    }
                }
            }
        }

        Err(format!(
            "no input arguments found for output {}",
            target_output
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut prog = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        prog.execute().unwrap();
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], prog);
    }

    #[test]
    fn test_example_2() {
        let mut prog = vec![1, 0, 0, 0, 99];
        prog.execute().unwrap();
        assert_eq!(vec![2, 0, 0, 0, 99], prog);
    }

    #[test]
    fn test_example_3() {
        let mut prog = vec![2, 3, 0, 3, 99];
        prog.execute().unwrap();
        assert_eq!(vec![2, 3, 0, 6, 99], prog);
    }

    #[test]
    fn test_example_4() {
        let mut prog = vec![2, 4, 4, 5, 99, 0];
        prog.execute().unwrap();
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], prog);
    }

    #[test]
    fn test_example_5() {
        let mut prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        prog.execute().unwrap();
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], prog);
    }

    #[test]
    fn answer_part_1() {
        let mut prog = get_gravity_assist_program();
        prog.execute_with_args(12, 2).unwrap();
        assert_eq!(3267740, prog[0]);
    }

    #[test]
    fn answer_part_2() {
        let prog = get_gravity_assist_program();
        let result = prog.search_for_output(19690720).unwrap();
        assert_eq!(78, result.0);
        assert_eq!(70, result.1);
        assert_eq!(7870, 100 * result.0 + result.1);
    }

    /// This program outputs whatever it gets as input, then halts.
    #[test]
    fn test_io() {
        let mut prog = vec![3, 0, 4, 0, 99];
        let mut exec = IntCodeProgramExecutor::from(&mut prog)
            .input(Input::new(27))
            .output(Output::new());
        exec.execute().unwrap();
        assert_eq!(exec.output.unwrap().value, 27);
    }

    #[test]
    fn test_parameter_modes() {
        let mut prog = vec![1002, 4, 3, 4, 33];
        prog.execute().unwrap();
        assert_eq!(prog, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut prog = vec![1101, 100, -1, 4, 0];
        prog.execute().unwrap();
        assert_eq!(prog, vec![1101, 100, -1, 4, 99]);
    }
}

pub fn get_gravity_assist_program() -> Vec<MemContent> {
    vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 1, 6, 23, 27,
        1, 27, 5, 31, 2, 31, 10, 35, 2, 35, 6, 39, 1, 39, 5, 43, 2, 43, 9, 47, 1, 47, 6, 51, 1, 13,
        51, 55, 2, 9, 55, 59, 1, 59, 13, 63, 1, 6, 63, 67, 2, 67, 10, 71, 1, 9, 71, 75, 2, 75, 6,
        79, 1, 79, 5, 83, 1, 83, 5, 87, 2, 9, 87, 91, 2, 9, 91, 95, 1, 95, 10, 99, 1, 9, 99, 103,
        2, 103, 6, 107, 2, 9, 107, 111, 1, 111, 5, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1, 123, 2,
        127, 1, 127, 9, 0, 99, 2, 0, 14, 0,
    ]
}
