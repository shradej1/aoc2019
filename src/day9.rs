use crate::day2::IntCodeProgramExecutor;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_relative_base() {
        let orig_prog = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut prog = orig_prog.clone();
        let mut exec = IntCodeProgramExecutor::from(&mut prog);
        exec.execute().unwrap();

        assert_eq!(orig_prog, exec.output);
    }
}
