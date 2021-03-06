use std::collections::BTreeSet;
use std::convert::TryFrom;

pub fn get_paths() -> (Vec<Movement>, Vec<Movement>) {
    let p1 = "R999, D666, L86, U464, R755, U652, R883, D287, L244, U308, L965, U629, R813, U985, \
              R620, D153, L655, D110, R163, D81, L909, D108, L673, D165, L620, U901, R601, D561, \
              L490, D21, R223, U478, R80, U379, R873, U61, L674, D732, R270, U297, L354, U264, \
              L615, D2, R51, D582, R280, U173, R624, U644, R451, D97, R209, U245, R32, U185, \
              R948, D947, R380, D945, L720, U305, R911, U614, L419, D751, L934, U371, R291, \
              D166, L137, D958, R368, U441, R720, U822, R961, D32, R242, D972, L782, D166, L680, \
              U111, R379, D155, R213, U573, R761, D543, R762, U953, R317, U841, L38, U900, R573, \
              U766, R807, U950, R945, D705, R572, D994, L633, U33, L173, U482, R253, D835, R800, \
              U201, L167, U97, R375, D813, L468, D924, L972, U570, R975, D898, L195, U757, L565, \
              D378, R935, U4, L334, D707, R958, U742, R507, U892, R174, D565, L862, D311, L770, \
              D619, L319, D698, L169, D652, L761, D644, R837, U43, L197, D11, L282, D345, L551, \
              U460, R90, D388, R911, U602, L21, D275, L763, U880, R604, D838, R146, U993, L99, \
              U99, R928, U54, L148, D863, R618, U449, R549, D659, R449, D435, L978, D612, L645, \
              D691, R190, D434, L841, D364, L634, D590, R962, U15, R921, D442, L284, U874, R475, \
              D556, L135, U376, L459, D673, L515, U438, L736, U266, L601, U351, R496, U891, L893, \
              D597, L135, D966, R121, U763, R46, D110, R830, U644, L932, D122, L123, U145, R273, \
              U690, L443, D372, R818, D259, L695, U69, R73, D718, R106, U929, L346, D291, L857, \
              D341, R297, D823, R819, U496, L958, U394, R102, D763, L444, D835, L33, U45, R812, \
              U845, R196, U458, R231, U637, R661, D983, L941, U975, L353, U609, L698, U152, R122, \
              D882, R682, D926, R729, U429, R255, D227, R987, D547, L446, U217, R678, D464, R849, \
              D472, L406, U940, L271, D779, R980, D751, L171, D420, L49, D271, R430, D530, R509, \
              U479, R135, D770, R85, U815, R328, U234, R83";

    let p2 = "L1008, D951, L618, U727, L638, D21, R804, D19, L246, U356, L51, U8, L627, U229, \
              R719, D198, L342, U240, L738, D393, L529, D22, R648, D716, L485, U972, L580, U884, \
              R612, D211, L695, U731, R883, U470, R732, U723, R545, D944, R18, U554, L874, D112, \
              R782, D418, R638, D296, L123, U426, L479, U746, L209, D328, L121, D496, L172, D228, \
              L703, D389, R919, U976, R364, D468, L234, U318, R912, U236, R148, U21, R26, D116, \
              L269, D913, L949, D206, L348, U496, R208, U706, R450, U472, R637, U884, L8, U82, \
              L77, D737, L677, D358, L351, U719, R154, U339, L506, U76, L952, D791, L64, U879, \
              R332, D244, R638, D453, L107, D908, L58, D188, R440, D147, R913, U298, L681, D582, \
              L943, U503, L6, U459, L289, D131, L739, D443, R333, D138, R553, D73, L475, U930, \
              L332, U518, R614, D553, L515, U602, R342, U95, R131, D98, R351, U921, L141, U207, \
              R199, U765, R55, U623, R768, D620, L722, U31, L891, D862, R85, U271, R590, D184, \
              R960, U149, L985, U82, R591, D384, R942, D670, R584, D637, L548, U844, R353, U496, \
              L504, U3, L830, U239, R246, U279, L146, U965, R784, U448, R60, D903, R490, D831, \
              L537, U109, R271, U306, L342, D99, L234, D936, R621, U870, R56, D29, R366, D562, \
              R276, D134, L289, D425, R597, D102, R276, D600, R1, U322, L526, D744, L259, D111, \
              R994, D581, L973, D871, R173, D924, R294, U478, R384, D242, R606, U629, R472, D651, \
              R526, U55, R885, U637, R186, U299, R812, D95, R390, D689, R514, U483, R471, D591, \
              L610, D955, L599, D674, R766, U834, L417, U625, R903, U376, R991, U175, R477, U524, \
              L453, D407, R72, D217, L968, D892, L806, D589, R603, U938, L942, D940, R578, U820, \
              L888, U232, L740, D348, R445, U269, L170, U979, L159, U433, L31, D818, L914, U600, \
              L33, U159, R974, D983, L922, U807, R682, U525, L234, U624, L973, U123, L875, D64, \
              L579, U885, L911, D578, R17, D293, L211";
    (parse_path(&p1), parse_path(&p2))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Movement {
    dir: Direction,
    amount: usize,
}

impl Movement {
    fn new(dir: Direction, amount: usize) -> Movement {
        Movement { dir, amount }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Path {
    fn iterator(&self) -> PathIterator;
    fn path_length(&self) -> usize;
}

impl Path for Vec<Movement> {
    fn iterator(&self) -> PathIterator {
        PathIterator {
            path: self,
            comp_idx: 0,
            progress: 0,
            coords: PathPosition(0, 0),
        }
    }

    fn path_length(&self) -> usize {
        self.iter().map(|c| c.amount).sum()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct PathPosition(i32, i32);

pub struct PathIterator<'a> {
    /// The path that is being iterated over
    path: &'a Vec<Movement>,

    /// The index of the "active" component
    comp_idx: usize,

    /// The progress towards completing the path segment defined by the active component.
    progress: usize,

    /// The current coordinates
    coords: PathPosition,
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = PathPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let coords = self.coords;
        if self.comp_idx < self.path.len() {
            let curr_comp = self.path[self.comp_idx];
            let next = match curr_comp.dir {
                Direction::Up => PathPosition(coords.0 + 1, coords.1),
                Direction::Down => PathPosition(coords.0 - 1, coords.1),
                Direction::Left => PathPosition(coords.0, coords.1 - 1),
                Direction::Right => PathPosition(coords.0, coords.1 + 1),
            };

            // update iterator state to advance progress on current component, or reset and move
            // to next component.
            self.coords = next;
            self.progress += 1;
            if self.progress == curr_comp.amount {
                self.progress = 0;
                self.comp_idx += 1;
            }

            Some(next)
        } else {
            None
        }
    }
}

pub fn intersect<P: Path>(p1: &P, p2: &P) -> Vec<PathPosition> {
    let p1: BTreeSet<_> = p1.iterator().collect();
    let p2: BTreeSet<_> = p2.iterator().collect();

    p1.intersection(&p2).cloned().collect()
}

fn parse_path(s: &str) -> Vec<Movement> {
    let mut movement = Vec::new();
    for tok in s.split(',') {
        let tok = tok.trim();
        let dir = tok.chars().nth(0).unwrap();
        let dist = *&tok[1..].parse::<usize>().unwrap();
        let mv = match dir {
            'L' => Movement::new(Direction::Left, dist),
            'R' => Movement::new(Direction::Right, dist),
            'U' => Movement::new(Direction::Up, dist),
            'D' => Movement::new(Direction::Down, dist),
            _ => panic!(format!("Unexpected direction: {}", dir)),
        };
        movement.push(mv);
    }
    movement
}

pub fn compute_min_manhattan_distance(pos: &Vec<PathPosition>) -> Option<i32> {
    pos.into_iter()
        .map(|PathPosition(x, y)| x.abs() + y.abs())
        .min()
}

pub fn compute_min_total_steps(
    intersections: &Vec<PathPosition>,
    p1: &Vec<Movement>,
    p2: &Vec<Movement>,
) -> Option<i32> {
    intersections
        .into_iter()
        .map(|p| {
            let p1_pos = p1
                .iterator()
                .enumerate()
                .find(|(_idx, comp)| comp == p)
                .unwrap()
                .0;
            let p2_pos = p2
                .iterator()
                .enumerate()
                .find(|(_idx, comp)| comp == p)
                .unwrap()
                .0;
            // add 2 since the index is the length of the path traversed minus 1.
            (p1_pos + p2_pos + 2) as i32
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paths() {
        let (p1, p2) = get_paths();
        assert_eq!(Movement::new(Direction::Left, 1008), p2[0]);
        assert_eq!(Movement::new(Direction::Down, 951), p2[1]);
    }

    #[test]
    fn test_path_length() {
        let p1 = parse_path("U5");
        assert_eq!(5, p1.iterator().into_iter().count());
    }

    #[test]
    fn test_example_0() {
        let p1 = parse_path("R8, U5, L5, D3");
        let p2 = parse_path("U7, R6, D4, L4");
        let intersections = intersect(&p1, &p2);
        assert_eq!(compute_min_manhattan_distance(&intersections).unwrap(), 6);
        assert_eq!(
            compute_min_total_steps(&intersections, &p1, &p2).unwrap(),
            30
        );
    }

    #[test]
    fn test_example_1() {
        let p1 = parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let p2 = parse_path("U62,R66,U55,R34,D71,R55,D58,R83");
        let intersections = intersect(&p1, &p2);
        assert_eq!(159, compute_min_manhattan_distance(&intersections).unwrap());
        assert_eq!(
            610,
            compute_min_total_steps(&intersections, &p1, &p2).unwrap()
        );
    }

    #[test]
    fn test_example_2() {
        let p1 = parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        assert_eq!(
            98 + 47 + 26 + 63 + 33 + 87 + 62 + 20 + 33 + 53 + 51,
            p1.path_length()
        );

        let p2 = parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let intersections = intersect(&p1, &p2);
        assert_eq!(135, compute_min_manhattan_distance(&intersections).unwrap());
        assert_eq!(
            410,
            compute_min_total_steps(&intersections, &p1, &p2).unwrap()
        );
    }

    #[test]
    fn day3_answer() {
        // computes the shortest manhattan distance path
        let (p1, p2) = get_paths();
        let intersections = intersect(&p1, &p2);
        let min_manhattan = compute_min_manhattan_distance(&intersections);
        assert_eq!(1983, min_manhattan.unwrap());

        // computes the minimum total steps
        let min_steps = compute_min_total_steps(&intersections, &p1, &p2);
        assert_eq!(107754, min_steps.unwrap());
    }
}
