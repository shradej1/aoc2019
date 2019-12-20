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
    value: Vec<MemContent>,
}

impl Output {
    fn new() -> Self {
        Output { value: Vec::new() }
    }
}

impl Output {
    fn write(&mut self, value: MemContent) {
        self.value.push(value)
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
        assert_eq!(exec.output.unwrap().value[0], 27);
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

    #[test]
    fn run_test_diagnostics() {
        let mut prog = get_test_diagnostic_program();
        let mut exec = IntCodeProgramExecutor::from(&mut prog)
            .input(Input::new(1))
            .output(Output::new());
        exec.execute().unwrap();
        let diagnostic_code = 13285749;
        assert_eq!(
            exec.output.unwrap().value,
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, diagnostic_code]
        );
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

pub fn get_test_diagnostic_program() -> Vec<MemContent> {
    vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 34, 7, 225, 101, 17, 169, 224, 1001,
        224, -92, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 6, 224, 1, 224, 223, 223, 1102, 46,
        28, 225, 1102, 66, 83, 225, 2, 174, 143, 224, 1001, 224, -3280, 224, 4, 224, 1002, 223, 8,
        223, 1001, 224, 2, 224, 1, 224, 223, 223, 1101, 19, 83, 224, 101, -102, 224, 224, 4, 224,
        102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1001, 114, 17, 224, 1001, 224, -63,
        224, 4, 224, 1002, 223, 8, 223, 1001, 224, 3, 224, 1, 223, 224, 223, 1102, 60, 46, 225,
        1101, 7, 44, 225, 1002, 40, 64, 224, 1001, 224, -1792, 224, 4, 224, 102, 8, 223, 223, 101,
        4, 224, 224, 1, 223, 224, 223, 1101, 80, 27, 225, 1, 118, 44, 224, 101, -127, 224, 224, 4,
        224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1102, 75, 82, 225, 1101, 40, 41,
        225, 1102, 22, 61, 224, 1001, 224, -1342, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 6, 224,
        1, 223, 224, 223, 102, 73, 14, 224, 1001, 224, -511, 224, 4, 224, 1002, 223, 8, 223, 101,
        5, 224, 224, 1, 224, 223, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 329, 1001, 223, 1, 223,
        1007, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 344, 101, 1, 223, 223, 1008, 226, 226,
        224, 1002, 223, 2, 223, 1006, 224, 359, 101, 1, 223, 223, 8, 226, 677, 224, 102, 2, 223,
        223, 1006, 224, 374, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1005, 224,
        389, 101, 1, 223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 404, 1001, 223, 1,
        223, 1108, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 419, 1001, 223, 1, 223, 1107, 677,
        677, 224, 102, 2, 223, 223, 1006, 224, 434, 1001, 223, 1, 223, 1108, 226, 677, 224, 1002,
        223, 2, 223, 1006, 224, 449, 101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005,
        224, 464, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 479, 1001,
        223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 494, 101, 1, 223, 223, 108,
        677, 677, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223, 7, 677, 226, 224,
        1002, 223, 2, 223, 1006, 224, 524, 101, 1, 223, 223, 1007, 677, 677, 224, 1002, 223, 2,
        223, 1006, 224, 539, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1006, 224,
        554, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 569, 1001, 223, 1,
        223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 584, 101, 1, 223, 223, 108, 226,
        226, 224, 102, 2, 223, 223, 1006, 224, 599, 1001, 223, 1, 223, 7, 226, 226, 224, 102, 2,
        223, 223, 1006, 224, 614, 1001, 223, 1, 223, 8, 226, 226, 224, 1002, 223, 2, 223, 1006,
        224, 629, 1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 644, 101, 1,
        223, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 659, 101, 1, 223, 223, 107,
        226, 677, 224, 102, 2, 223, 223, 1006, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
    ]
}
