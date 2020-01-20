use crate::day2::{IntCodeProgram, IntCodeProgramExecutor, MemContent, ProgramState};
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Color {
    Black,
    White,
}

impl From<Color> for MemContent {
    fn from(c: Color) -> MemContent {
        match c {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl From<MemContent> for Color {
    fn from(v: MemContent) -> Color {
        match v {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unexpected color: {}", v),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn left(&self) -> Self {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
            Orientation::Right => Orientation::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
}

impl From<MemContent> for Direction {
    fn from(v: MemContent) -> Direction {
        match v {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => panic!("Unexpected direction: {}", v),
        }
    }
}

#[derive(Debug, Clone)]
struct Hull {
    tiles: BTreeMap<(i32, i32), Color>,
}

impl Hull {
    fn new() -> Hull {
        Hull {
            tiles: BTreeMap::new(),
        }
    }

    fn painted_count(&self) -> usize {
        self.tiles.len()
    }

    fn color(&self, tile: &(i32, i32)) -> Color {
        self.tiles.get(tile).cloned().unwrap_or(Color::Black)
    }

    fn paint(&mut self, tile: (i32, i32), c: Color) {
        self.tiles.insert(tile, c);
    }
}

#[derive(Debug, Clone)]
struct Robot {
    orientation: Orientation,
    pos: (i32, i32),
}

impl Robot {
    fn new() -> Robot {
        Robot {
            orientation: Orientation::Up,
            pos: (0, 0),
        }
    }

    fn forward(&mut self) {
        match self.orientation {
            Orientation::Up => self.pos.1 += 1,
            Orientation::Down => self.pos.1 -= 1,
            Orientation::Left => self.pos.0 -= 1,
            Orientation::Right => self.pos.0 += 1,
        }
    }

    fn turn(&mut self, dir: Direction) {
        self.orientation = match dir {
            Direction::Left => self.orientation.left(),
            Direction::Right => self.orientation.right(),
        }
    }
}

fn execute() -> usize {
    let mut robot = Robot::new();
    let mut hull = Hull::new();

    let mut prog = get_program();
    let mut exec = IntCodeProgramExecutor::from(&mut prog);
    let mut state = exec.execute();
    while let Ok(ref inner) = state {
        match inner {
            ProgramState::AwaitingInput => {
                state = exec.resume(hull.color(&robot.pos).into());
                let color = exec.output.remove(0);

                let direction = exec.output.remove(0);
                assert!(exec.output.is_empty());
                hull.paint(robot.pos.clone(), color.into());
                robot.turn(direction.into());
                robot.forward();
            }
            ProgramState::Terminated(out) => break,
        }
    }

    state.expect("IntCode Program terminated abnormally");

    hull.painted_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_left() {
        let mut robot = Robot::new();
        assert_eq!(Orientation::Up, robot.orientation);

        robot.turn(Direction::Left);
        assert_eq!(Orientation::Left, robot.orientation);
        assert_eq!((0, 0), robot.pos);
        robot.forward();
        assert_eq!((-1, 0), robot.pos);
    }

    #[test]
    fn test_turn_right() {
        let mut robot = Robot::new();
        robot.turn(Direction::Right);
        assert_eq!(Orientation::Right, robot.orientation);
        assert_eq!((0, 0), robot.pos);
        robot.forward();
        assert_eq!((1, 0), robot.pos);
    }

    #[test]
    fn test_example() {
        let mut robot = Robot::new();
        let mut hull = Hull::new();

        // 1, 0
        hull.paint(robot.pos, Color::White);
        robot.turn(Direction::Left);
        robot.forward();

        // 0, 0
        hull.paint(robot.pos, Color::Black);
        robot.turn(Direction::Left);
        robot.forward();

        // 1, 0
        hull.paint(robot.pos, Color::White);
        robot.turn(Direction::Left);
        robot.forward();

        // 1, 0
        hull.paint(robot.pos, Color::White);
        robot.turn(Direction::Left);
        robot.forward();

        // 0, 1
        hull.paint(robot.pos, Color::Black);
        robot.turn(Direction::Right);
        robot.forward();

        // 1, 0
        hull.paint(robot.pos, Color::White);
        robot.turn(Direction::Left);
        robot.forward();

        // 1, 0
        hull.paint(robot.pos, Color::White);
        robot.turn(Direction::Left);
        robot.forward();

        assert_eq!(6, hull.painted_count());

        assert_eq!(hull.color(&(-1, -1)), Color::White);
        assert_eq!(hull.color(&(0, -1)), Color::White);
        assert_eq!(hull.color(&(1, 0)), Color::White);
        assert_eq!(hull.color(&(1, 1)), Color::White);
    }

    #[test]
    fn test_hull_color() {
        let mut hull = Hull::new();
        assert_eq!(0, hull.painted_count());

        hull.paint((0, 0), Color::Black);
        assert_eq!(1, hull.painted_count());
    }

    #[test]
    fn test_execute() {
        assert_eq!(2172, execute());
    }
}

fn get_program() -> Vec<MemContent> {
    vec![
        3,
        8,
        1005,
        8,
        342,
        1106,
        0,
        11,
        0,
        0,
        0,
        104,
        1,
        104,
        0,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        1002,
        8,
        1,
        29,
        2,
        1006,
        19,
        10,
        1,
        1005,
        19,
        10,
        2,
        1102,
        11,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1001,
        8,
        0,
        62,
        2,
        1009,
        15,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        88,
        2,
        1101,
        6,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        114,
        1,
        105,
        8,
        10,
        1,
        1102,
        18,
        10,
        2,
        6,
        5,
        10,
        1,
        2,
        15,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        1001,
        8,
        0,
        153,
        1,
        105,
        15,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        178,
        1,
        1006,
        15,
        10,
        1006,
        0,
        96,
        1006,
        0,
        35,
        1,
        104,
        7,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        214,
        1006,
        0,
        44,
        2,
        1105,
        17,
        10,
        1,
        1107,
        19,
        10,
        1,
        4,
        16,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        102,
        1,
        8,
        252,
        1006,
        0,
        6,
        1,
        1001,
        20,
        10,
        1006,
        0,
        45,
        2,
        1109,
        5,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        287,
        2,
        101,
        20,
        10,
        2,
        1006,
        18,
        10,
        1,
        1009,
        9,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        321,
        101,
        1,
        9,
        9,
        1007,
        9,
        1031,
        10,
        1005,
        10,
        15,
        99,
        109,
        664,
        104,
        0,
        104,
        1,
        21102,
        48210117528,
        1,
        1,
        21102,
        1,
        359,
        0,
        1105,
        1,
        463,
        21102,
        932700763028,
        1,
        1,
        21102,
        370,
        1,
        0,
        1105,
        1,
        463,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        21102,
        1,
        179557207079,
        1,
        21102,
        417,
        1,
        0,
        1105,
        1,
        463,
        21102,
        1,
        28994202816,
        1,
        21101,
        0,
        428,
        0,
        1105,
        1,
        463,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        0,
        21101,
        0,
        709580710756,
        1,
        21102,
        1,
        451,
        0,
        1106,
        0,
        463,
        21102,
        825016201984,
        1,
        1,
        21101,
        462,
        0,
        0,
        1106,
        0,
        463,
        99,
        109,
        2,
        21201,
        -1,
        0,
        1,
        21102,
        40,
        1,
        2,
        21101,
        0,
        494,
        3,
        21102,
        1,
        484,
        0,
        1105,
        1,
        527,
        109,
        -2,
        2106,
        0,
        0,
        0,
        1,
        0,
        0,
        1,
        109,
        2,
        3,
        10,
        204,
        -1,
        1001,
        489,
        490,
        505,
        4,
        0,
        1001,
        489,
        1,
        489,
        108,
        4,
        489,
        10,
        1006,
        10,
        521,
        1101,
        0,
        0,
        489,
        109,
        -2,
        2105,
        1,
        0,
        0,
        109,
        4,
        1201,
        -1,
        0,
        526,
        1207,
        -3,
        0,
        10,
        1006,
        10,
        544,
        21102,
        1,
        0,
        -3,
        21202,
        -3,
        1,
        1,
        22102,
        1,
        -2,
        2,
        21102,
        1,
        1,
        3,
        21102,
        563,
        1,
        0,
        1105,
        1,
        568,
        109,
        -4,
        2106,
        0,
        0,
        109,
        5,
        1207,
        -3,
        1,
        10,
        1006,
        10,
        591,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        591,
        21202,
        -4,
        1,
        -4,
        1105,
        1,
        659,
        22102,
        1,
        -4,
        1,
        21201,
        -3,
        -1,
        2,
        21202,
        -2,
        2,
        3,
        21102,
        610,
        1,
        0,
        1106,
        0,
        568,
        21201,
        1,
        0,
        -4,
        21102,
        1,
        1,
        -1,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        629,
        21102,
        1,
        0,
        -1,
        22202,
        -2,
        -1,
        -2,
        2107,
        0,
        -3,
        10,
        1006,
        10,
        651,
        21202,
        -1,
        1,
        1,
        21102,
        1,
        651,
        0,
        106,
        0,
        526,
        21202,
        -2,
        -1,
        -2,
        22201,
        -4,
        -2,
        -4,
        109,
        -5,
        2106,
        0,
        0,
    ]
}
