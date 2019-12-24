use crate::day2::*;
use std::collections::btree_set::BTreeSet;
use std::iter::FromIterator;

struct AmplifierControllerSoftware {
    program: Vec<MemContent>,
    phase_setting: MemContent,
    input_signal: MemContent,
}

impl AmplifierControllerSoftware {
    fn new(program: Vec<MemContent>, phase_setting: MemContent, input_signal: MemContent) -> Self {
        AmplifierControllerSoftware {
            program,
            phase_setting,
            input_signal,
        }
    }

    fn default(phase_setting: MemContent, input_signal: MemContent) -> Self {
        AmplifierControllerSoftware {
            program: get_amplifier_controller_software(),
            phase_setting,
            input_signal,
        }
    }

    fn execute(&mut self) -> MemContent {
        let mut exec = IntCodeProgramExecutor::from(&mut self.program);
        exec.mut_input()
            .append(&mut vec![self.phase_setting, self.input_signal]);
        exec.execute().unwrap();
        exec.output()[0]
    }
}

fn compute_thruster_signal(prog: &Vec<MemContent>, phase_settings: &[MemContent]) -> MemContent {
    assert!(phase_settings.len() == 5);
    let a_out = AmplifierControllerSoftware::new(prog.clone(), phase_settings[0], 0).execute();
    let b_out = AmplifierControllerSoftware::new(prog.clone(), phase_settings[1], a_out).execute();
    let c_out = AmplifierControllerSoftware::new(prog.clone(), phase_settings[2], b_out).execute();
    let d_out = AmplifierControllerSoftware::new(prog.clone(), phase_settings[3], c_out).execute();
    let e_out = AmplifierControllerSoftware::new(prog.clone(), phase_settings[4], d_out).execute();
    e_out
}

fn permute(set: &BTreeSet<MemContent>) -> Vec<Vec<MemContent>> {
    if set.len() == 1 {
        vec![set.iter().cloned().collect()]
    } else {
        let mut permutations: Vec<Vec<MemContent>> = Vec::new();
        for elem in set {
            let mut o = set.clone();
            o.remove(elem);
            for p in permute(&o) {
                let mut perm = vec![*elem];
                perm.extend(p);
                permutations.push(perm);
            }
        }
        permutations
    }
}

fn compute_thruster_signal_with_feedback(
    prog: &Vec<MemContent>,
    phase_settings: &[MemContent],
) -> MemContent {
    assert!(phase_settings.len() == 5);

    let mut p = prog.clone();
    let mut amp_a = IntCodeProgramExecutor::from(&mut p);
    amp_a.mut_input().push(phase_settings[0]);

    let mut p = prog.clone();
    let mut amp_b = IntCodeProgramExecutor::from(&mut p);
    amp_b.mut_input().push(phase_settings[1]);

    let mut p = prog.clone();
    let mut amp_c = IntCodeProgramExecutor::from(&mut p);
    amp_c.mut_input().push(phase_settings[2]);

    let mut p = prog.clone();
    let mut amp_d = IntCodeProgramExecutor::from(&mut p);
    amp_d.mut_input().push(phase_settings[3]);

    let mut p = prog.clone();
    let mut amp_e = IntCodeProgramExecutor::from(&mut p);
    amp_e.mut_input().push(phase_settings[4]);

    amp_a.mut_input().push(0);
    loop {
        if let ProgramState::Terminated(_) = amp_a.execute().unwrap() {
            println!("Amplifier A terminated.");
        }
        amp_b.mut_input().push(amp_a.output.remove(0));

        if let ProgramState::Terminated(_) = amp_b.execute().unwrap() {
            println!("Amplifier B terminated.");
        }
        amp_c.mut_input().push(amp_b.output.remove(0));

        if let ProgramState::Terminated(_) = amp_c.execute().unwrap() {
            println!("Amplifier C terminated.");
        }
        amp_d.mut_input().push(amp_c.output.remove(0));

        if let ProgramState::Terminated(_) = amp_d.execute().unwrap() {
            println!("Amplifier D terminated.");
        }
        amp_e.mut_input().push(amp_d.output.remove(0));

        match amp_e.execute().unwrap() {
            ProgramState::Terminated(result) => return amp_e.output.remove(0),
            ProgramState::AwaitingInput => amp_a.mut_input().push(amp_e.output.remove(0)),
        }
    }
}

/// Searches for the phase settings (0, 1, 2, 3, 4) that maximize the thrust program output.
/// Each phase setting is only used once.
fn search_max_thrust_signal_settings() -> [MemContent; 5] {
    let mut max = [0, 1, 2, 3, 4];
    let mut max_output = 0;
    let prog = get_amplifier_controller_software();

    // TODO: iterate over all thrust combinations

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let test_prog = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(compute_thruster_signal(&test_prog, &[4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn test_example_2() {
        let test_prog = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(compute_thruster_signal(&test_prog, &[0, 1, 2, 3, 4]), 54321);
    }

    #[test]
    fn test_example_3() {
        let test_prog = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(compute_thruster_signal(&test_prog, &[1, 0, 4, 3, 2]), 65210);
    }

    #[test]
    fn test_cycle_phase_settings() {
        let set: BTreeSet<MemContent> = [0, 1, 2, 3, 4].iter().cloned().collect();
        let permutations = permute(&set);
        assert_eq!(5 * 4 * 3 * 2 * 1, permutations.len());
    }

    #[test]
    fn find_optimal_phase_settings() {
        let prog = get_amplifier_controller_software();
        let set: BTreeSet<MemContent> = [0, 1, 2, 3, 4].iter().cloned().collect();
        let permutations = permute(&set);
        let mut max_perm = &permutations[0];
        let mut max_output = 0;

        for p in &permutations {
            let output = compute_thruster_signal(&prog, &p);
            if output > max_output {
                max_output = output;
                max_perm = &p;
            }
        }

        assert_eq!(422858, max_output);
        assert_eq!(&[3, 1, 4, 2, 0], &max_perm[..]);
    }

    #[test]
    fn test_compute_thruster_signal_with_feedback() {
        let prog = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            139629729,
            compute_thruster_signal_with_feedback(&prog, &[9, 8, 7, 6, 5])
        );
    }

    #[test]
    fn test_compute_thruster_signal_with_feedback_2() {
        let prog = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(
            18216,
            compute_thruster_signal_with_feedback(&prog, &[9, 7, 8, 5, 6])
        );
    }

    #[test]
    fn find_optimal_phase_settings_with_feedback() {
        let prog = get_amplifier_controller_software();
        let set: BTreeSet<MemContent> = [5, 6, 7, 8, 9].iter().cloned().collect();
        let permutations = permute(&set);
        let mut max_perm = &permutations[0];
        let mut max_output = 0;

        for p in &permutations {
            let output = compute_thruster_signal_with_feedback(&prog, &p);
            if output > max_output {
                max_output = output;
                max_perm = &p;
            }
        }

        assert_eq!(14897241, max_output);
        assert_eq!(&[7, 8, 9, 6, 5], &max_perm[..]);
    }
}

fn get_amplifier_controller_software() -> Vec<MemContent> {
    vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 34, 51, 76, 101, 126, 207, 288, 369, 450, 99999, 3,
        9, 102, 4, 9, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 1002, 9, 3, 9, 101, 3, 9, 9,
        4, 9, 99, 3, 9, 102, 5, 9, 9, 1001, 9, 2, 9, 102, 2, 9, 9, 101, 3, 9, 9, 1002, 9, 2, 9, 4,
        9, 99, 3, 9, 101, 5, 9, 9, 102, 5, 9, 9, 1001, 9, 2, 9, 102, 3, 9, 9, 1001, 9, 3, 9, 4, 9,
        99, 3, 9, 101, 2, 9, 9, 1002, 9, 5, 9, 1001, 9, 5, 9, 1002, 9, 4, 9, 101, 5, 9, 9, 4, 9,
        99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
        3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3,
        9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9,
        1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
    ]
}
