/// Implements an Intcode computer
use std::convert::TryFrom;
use std::convert::TryInto;

pub type Result<T> = std::result::Result<T, String>;
pub type MemContent = i64;
pub type Addr = usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ParameterMode {
    /// Parameters are interpreted as a position.  If the parameter is 50, its value is the value
    /// stored at address 50 in memory.
    PositionMode,

    /// Parameters are interpreted as values.  If the parameter is 50, the value is simply 50.
    ImmediateMode,

    /// Parameters in relative mode are read like position mode, but with a different starting base
    /// address
    RelativeMode,
}

impl ParameterMode {
    // TODO: move into executor
    fn parse(&self, prog: &Vec<MemContent>, loc: Addr, relative_base: Addr) -> MemContent {
        fn read_or_default(prog: &Vec<MemContent>, abs_addr: Addr) -> MemContent {
            if abs_addr > prog.len() {
                0
            } else {
                prog[abs_addr]
            }
        }

        match self {
            ParameterMode::PositionMode => {
                let read_pos = usize::try_from(read_or_default(prog, loc)).unwrap();
                read_or_default(prog, read_pos)
            }
            ParameterMode::ImmediateMode => read_or_default(prog, loc),
            ParameterMode::RelativeMode => {
                let abs_loc;
                let offset = read_or_default(prog, loc);
                if offset < 0 {
                    abs_loc = relative_base
                        .checked_sub(usize::try_from(-offset).unwrap())
                        .unwrap();
                } else {
                    abs_loc = relative_base + usize::try_from(offset).unwrap();
                }

                read_or_default(prog, abs_loc)
            }
        }
    }
}

fn parse_parameter_value(
    prog: &Vec<MemContent>,
    instr_ptr: Addr,
    parameter_offset: usize,
    relative_base: usize,
) -> MemContent {
    let param_mode =
        ParameterMode::try_from(prog[instr_ptr] / 10_i64.pow(parameter_offset as u32 + 1) % 10)
            .unwrap();
    param_mode.parse(prog, instr_ptr + parameter_offset, relative_base)
}

fn parse_write_index(
    prog: &Vec<MemContent>,
    instr_ptr: Addr,
    parameter_offset: usize,
    relative_base: usize,
) -> Addr {
    let param_mode =
        ParameterMode::try_from(prog[instr_ptr] / 10_i64.pow(parameter_offset as u32 + 1) % 10)
            .unwrap();
    assert_ne!(param_mode, ParameterMode::ImmediateMode);
    // TODO: I think we should be able to use this...  but it doesn't work
    // param_mode
    //     .parse(prog, instr_ptr + parameter_offset, relative_base)
    //     .try_into()
    //     .unwrap()
    let offset = if param_mode == ParameterMode::RelativeMode {
        relative_base
    } else {
        0
    };

    usize::try_from(prog[instr_ptr + parameter_offset] + offset as i64).unwrap()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBaseOffsetAdj,
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
            5 => Ok(OpCode::JumpIfTrue),
            6 => Ok(OpCode::JumpIfFalse),
            7 => Ok(OpCode::LessThan),
            8 => Ok(OpCode::Equals),
            9 => Ok(OpCode::RelativeBaseOffsetAdj),
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
            2 => Ok(ParameterMode::RelativeMode),
            _ => Err(format!("Unexepcted parameter mode: {}", u)),
        }
    }
}

pub struct IntCodeProgramExecutor<T> {
    program: T,
    noun: MemContent,
    verb: MemContent,
    instr_ptr: Addr,
    relative_base: Addr,
    input: Vec<MemContent>,
    pub output: Vec<MemContent>,
}

impl From<Vec<MemContent>> for IntCodeProgramExecutor<Vec<MemContent>> {
    fn from(program: Vec<MemContent>) -> Self {
        let noun = program[1];
        let verb = program[2];
        IntCodeProgramExecutor {
            program,
            noun,
            verb,
            instr_ptr: 0,
            relative_base: 0,
            input: Vec::new(),
            output: Vec::new(),
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
            relative_base: 0,
            instr_ptr: 0,
            input: Vec::new(),
            output: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum ProgramState {
    AwaitingInput,
    Terminated(MemContent),
}

// TODO: remove this, let IntCodeProgramExecutor own program, and add accessor
impl IntCodeProgramExecutor<&mut Vec<MemContent>> {
    pub fn mut_input(&mut self) -> &mut Vec<MemContent> {
        &mut self.input
    }

    pub fn output(&self) -> &Vec<MemContent> {
        &self.output
    }

    pub fn resume(&mut self, input: MemContent) -> Result<ProgramState> {
        self.input.push(input);
        self.execute()
    }

    pub fn write(&mut self, loc: Addr, content: MemContent) {
        if loc >= self.program.len() {
            self.program.resize(loc + 1, MemContent::default());
        }
        self.program[loc] = content;
    }

    pub fn execute(&mut self) -> Result<ProgramState> {
        loop {
            // The opcode is a two-digit number based only on the ones and tens digit of the value
            let opcode = OpCode::try_from(self.program[self.instr_ptr] % 100)?;
            match opcode {
                OpCode::Add => {
                    let a1 = self.get_param(1);
                    let a2 = self.get_param(2);
                    let dest = self.get_write_index(3);

                    self.write(dest, a1 + a2);
                    self.instr_ptr += 4;
                }
                OpCode::Multiply => {
                    let a1 = self.get_param(1);
                    let a2 = self.get_param(2);
                    let dest = self.get_write_index(3);

                    self.write(dest, a1 * a2);
                    self.instr_ptr += 4;
                }
                OpCode::Input => {
                    println!("{}", self.program[self.instr_ptr]);
                    let store_addr = self.get_write_index(1);
                    if self.input.is_empty() {
                        return Ok(ProgramState::AwaitingInput);
                    }
                    let input = self.input.remove(0);
                    self.write(store_addr, input);
                    self.instr_ptr += 2;
                }
                OpCode::Output => {
                    let output_value = self.get_param(1);
                    self.output.push(output_value);
                    self.instr_ptr += 2;
                }
                OpCode::JumpIfTrue => {
                    let a1 = self.get_param(1);
                    let a2 = self.get_param(2);

                    if a1 != 0 {
                        self.instr_ptr = a2.try_into().unwrap();
                    } else {
                        self.instr_ptr += 3;
                    }
                }
                OpCode::JumpIfFalse => {
                    let a1 = self.get_param(1);
                    let a2 = self.get_param(2);

                    if a1 == 0 {
                        // instruction pointer modified.  do not advance instruction pointer
                        self.instr_ptr = a2.try_into().unwrap();
                    } else {
                        self.instr_ptr += 3;
                    }
                }
                OpCode::LessThan => {
                    let a1 = self.get_param(1);
                    let a2 = self.get_param(2);
                    let a3 = self.get_write_index(3);

                    self.write(a3, if a1 < a2 { 1 } else { 0 });
                    self.instr_ptr += 4;
                }
                OpCode::Equals => {
                    let a1 = self.get_param(1);
                    let a2 = self.get_param(2);
                    let a3 = self.get_write_index(3);

                    self.write(a3, if a1 == a2 { 1 } else { 0 });
                    self.instr_ptr += 4;
                }
                OpCode::RelativeBaseOffsetAdj => {
                    let orig = self.relative_base;
                    let adjustment = self.get_param(1);
                    if adjustment < 0 {
                        self.relative_base = self
                            .relative_base
                            .checked_sub(usize::try_from(-adjustment).unwrap())
                            .unwrap();
                    } else {
                        self.relative_base =
                            self.relative_base + usize::try_from(adjustment).unwrap();
                    }
                    self.instr_ptr += 2;
                }
                OpCode::Halt => break,
            }
        }
        Ok(ProgramState::Terminated(self.program[0]))
    }

    fn get_param(&self, param_offset: usize) -> MemContent {
        parse_parameter_value(
            self.program,
            self.instr_ptr,
            param_offset,
            self.relative_base,
        )
    }

    fn get_write_index(&self, param_offset: usize) -> Addr {
        parse_write_index(
            self.program,
            self.instr_ptr,
            param_offset,
            self.relative_base,
        )
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
        let state = IntCodeProgramExecutor::from(self).execute();
        if let Ok(ProgramState::Terminated(result)) = state {
            Ok(result)
        } else {
            panic!("Execution returned with unexpected state: {:?}", state);
        }
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
        let mut exec = IntCodeProgramExecutor::from(&mut prog);
        exec.mut_input().push(27);
        exec.execute().unwrap();
        assert_eq!(exec.output()[0], 27);
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
    fn run_test_diagnostics_air_conditioner_day5() {
        let mut prog = get_test_diagnostic_program();
        let mut exec = IntCodeProgramExecutor::from(&mut prog);
        exec.input.push(1);
        exec.execute().unwrap();
        let diagnostic_code = 13285749;
        assert_eq!(
            exec.output(),
            &vec![0, 0, 0, 0, 0, 0, 0, 0, 0, diagnostic_code]
        );
    }

    #[test]
    fn test_parse_parameter_value() {
        let prog = vec![1002, 4, 3, 4, 33];
        let instr_ptr = 0;
        let p1 = parse_parameter_value(&prog, instr_ptr, 1, 0);
        let p2 = parse_parameter_value(&prog, instr_ptr, 2, 0);
        let p3 = parse_write_index(&prog, instr_ptr, 3, 0);
        assert_eq!(p1, 33);
        assert_eq!(p2, 3);
        assert_eq!(p3, 4);
    }

    /// Executes a program that takes a single input value, and produces a single output value
    fn execute_with_input(prog: &Vec<MemContent>, input: MemContent) -> MemContent {
        let mut prog = prog.clone();
        let mut exec = IntCodeProgramExecutor::from(&mut prog);
        exec.input.push(input);
        exec.execute().unwrap();
        exec.output()[0]
    }

    #[test]
    fn test_position_mode_input_equal_to_8() {
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(0, execute_with_input(&prog, 1));
        assert_eq!(0, execute_with_input(&prog, 2));
        assert_eq!(1, execute_with_input(&prog, 8));
        assert_eq!(0, execute_with_input(&prog, 9));
    }

    #[test]
    fn test_position_mode_input_less_than_8() {
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(1, execute_with_input(&prog, 1));
        assert_eq!(1, execute_with_input(&prog, 2));
        assert_eq!(0, execute_with_input(&prog, 8));
        assert_eq!(0, execute_with_input(&prog, 9));
    }

    #[test]
    fn test_immediate_mode_input_equal_to_8() {
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        assert_eq!(0, execute_with_input(&prog, 1));
        assert_eq!(0, execute_with_input(&prog, 2));
        assert_eq!(1, execute_with_input(&prog, 8));
        assert_eq!(0, execute_with_input(&prog, 9));
    }

    #[test]
    fn test_immediate_mode_input_less_than_8() {
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        assert_eq!(1, execute_with_input(&prog, 1));
        assert_eq!(1, execute_with_input(&prog, 2));
        assert_eq!(0, execute_with_input(&prog, 8));
        assert_eq!(0, execute_with_input(&prog, 9));
    }

    #[test]
    fn test_position_mode_jump_test() {
        let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        assert_eq!(0, execute_with_input(&prog, 0));
        assert_eq!(1, execute_with_input(&prog, 2));
        assert_eq!(1, execute_with_input(&prog, -1));
        assert_eq!(1, execute_with_input(&prog, 1));
    }

    #[test]
    fn test_immediate_mode_jump_test() {
        let prog = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(0, execute_with_input(&prog, 0));
        assert_eq!(1, execute_with_input(&prog, 2));
        assert_eq!(1, execute_with_input(&prog, -1));
        assert_eq!(1, execute_with_input(&prog, 1));
    }

    #[test]
    fn test_below_equal_greater_than_8() {
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(999, execute_with_input(&prog, 7));
        assert_eq!(1000, execute_with_input(&prog, 8));
        assert_eq!(1001, execute_with_input(&prog, 9));
    }

    #[test]
    fn run_test_diagnostic_thermal_radiator_controller_day5() {
        let prog = get_test_diagnostic_program();
        assert_eq!(5000972, execute_with_input(&prog, 5));
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
