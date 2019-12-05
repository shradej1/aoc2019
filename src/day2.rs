/// Implements an Intcode computer
use std::convert::TryFrom;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum OpCode {
    Add,
    Multiply,
    Halt,
}

pub trait IntCodeProgram {
    fn execute(&mut self) -> Result<usize>;
    fn execute_with_args(&mut self, arg1: usize, arg2: usize) -> Result<usize>;
    fn output(&self) -> usize;
    fn search_for_output(&self, target_output: usize) -> Result<(usize, usize)>;
}

impl IntCodeProgram for Vec<usize> {
    fn execute(&mut self) -> Result<usize> {
        let mut instr_ptr = 0;
        loop {
            let opcode = OpCode::try_from(self[instr_ptr])?;
            match opcode {
                OpCode::Add => {
                    let a1 = self[self[instr_ptr + 1]];
                    let a2 = self[self[instr_ptr + 2]];
                    let dest: usize = self[instr_ptr + 3];
                    self[dest] = a1 + a2;
                }
                OpCode::Multiply => {
                    let a1 = self[self[instr_ptr + 1]];
                    let a2 = self[self[instr_ptr + 2]];
                    let dest: usize = self[instr_ptr + 3];
                    self[dest] = a1 * a2;
                }
                OpCode::Halt => break,
            }
            instr_ptr += 4;
        }
        Ok(self.output())
    }

    fn execute_with_args(&mut self, noun: usize, verb: usize) -> Result<usize> {
        self[1] = noun;
        self[2] = verb;
        self.execute()
    }

    fn output(&self) -> usize {
        self[0]
    }

    /// Returns the noun and verb that produce the given input
    ///
    /// Note that this is immutable since the original program state must be restored upon each
    /// execution.
    fn search_for_output(&self, target_output: usize) -> Result<(usize, usize)> {
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

impl TryFrom<usize> for OpCode {
    type Error = String;
    fn try_from(u: usize) -> Result<OpCode> {
        match u {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Multiply),
            99 => Ok(OpCode::Halt),
            _ => Err(format!("Unexpected opcode: {}", u)),
        }
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
}

pub fn get_gravity_assist_program() -> Vec<usize> {
    vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 1, 6, 23, 27,
        1, 27, 5, 31, 2, 31, 10, 35, 2, 35, 6, 39, 1, 39, 5, 43, 2, 43, 9, 47, 1, 47, 6, 51, 1, 13,
        51, 55, 2, 9, 55, 59, 1, 59, 13, 63, 1, 6, 63, 67, 2, 67, 10, 71, 1, 9, 71, 75, 2, 75, 6,
        79, 1, 79, 5, 83, 1, 83, 5, 87, 2, 9, 87, 91, 2, 9, 91, 95, 1, 95, 10, 99, 1, 9, 99, 103,
        2, 103, 6, 107, 2, 9, 107, 111, 1, 111, 5, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1, 123, 2,
        127, 1, 127, 9, 0, 99, 2, 0, 14, 0,
    ]
}
