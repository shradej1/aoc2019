use num::Integer;
use std::collections::BTreeSet;
use std::convert::TryFrom;

/// Location of an asteroid.
/// `x`: The distance from the left edge
/// `y`: The distance from the top edge
/// TODO: this should be a generic position, and AsteroidMap should define a `is_asteroid(pos) ->
/// bool` fn
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct AsteroidLocation {
    x: usize,
    y: usize,
}

impl AsteroidLocation {
    fn new(x: usize, y: usize) -> Self {
        AsteroidLocation { x, y }
    }
}

impl From<(usize, usize)> for AsteroidLocation {
    fn from(pos: (usize, usize)) -> Self {
        AsteroidLocation { x: pos.0, y: pos.1 }
    }
}

impl TryFrom<(i64, i64)> for AsteroidLocation {
    type Error = String;
    fn try_from(pos: (i64, i64)) -> Result<Self, String> {
        Ok(AsteroidLocation {
            x: usize::try_from(pos.0).map_err(|e| format!("{}", e))?,
            y: usize::try_from(pos.1).map_err(|e| format!("{}", e))?,
        })
    }
}

impl Into<(i64, i64)> for AsteroidLocation {
    fn into(self) -> (i64, i64) {
        (self.x as i64, self.y as i64)
    }
}

trait AsteroidMap {
    fn count_asteroids_visible_from(&self, pos: &AsteroidLocation) -> usize;
    fn clear_path_exists(&self, start: &AsteroidLocation, end: &AsteroidLocation) -> bool;
    fn find_first(
        &self,
        origin: &AsteroidLocation,
        direction: &AsteroidLocation,
    ) -> Option<AsteroidLocation>;
}

impl AsteroidMap for BTreeSet<AsteroidLocation> {
    fn count_asteroids_visible_from(&self, pos: &AsteroidLocation) -> usize {
        let mut count = 0;
        for other in self {
            if other != pos && self.clear_path_exists(pos, other) {
                count += 1;
            }
        }
        count
    }

    fn clear_path_exists(&self, start: &AsteroidLocation, end: &AsteroidLocation) -> bool {
        if let Some(first) = self.find_first(start, end) {
            first == *end
        } else {
            false
        }
    }

    fn find_first(
        &self,
        origin: &AsteroidLocation,
        direction: &AsteroidLocation,
    ) -> Option<AsteroidLocation> {
        if origin == direction {
            Some(*origin)
        } else {
            let start: (i64, i64) = origin.clone().into();
            let end: (i64, i64) = direction.clone().into();

            // compute slope and reduce
            let slope = (end.0 - start.0, end.1 - start.1);
            let slope = if slope.0 == 0 {
                (0, slope.1.signum())
            } else if slope.1 == 0 {
                (slope.0.signum(), 0)
            } else {
                let gcd = slope.0.gcd(&slope.1);
                (slope.0 / gcd, slope.1 / gcd)
            };

            let mut curr = (start.0 + slope.0, start.1 + slope.1);
            while curr != end {
                let maybe_asteroid = AsteroidLocation::try_from(curr).unwrap();
                if self.contains(&maybe_asteroid) {
                    return Some(maybe_asteroid);
                }
                curr = (curr.0 + slope.0, curr.1 + slope.1);
            }
            if curr == end {
                Some(AsteroidLocation::try_from(curr).unwrap())
            } else {
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct NonNan(f64);

impl NonNan {
    /// Panics if val is NaN
    fn new(val: f64) -> Self {
        assert!(!val.is_nan());
        NonNan(val)
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// This is defined so that straight up is PI radians, rotating clockwise to PI/2 to the right, 0
/// straight down, -PI/2 to the left, and approaching -PI coming back around to vertical.
fn compute_laser_angle(station_loc: &AsteroidLocation, asteroid_loc: &AsteroidLocation) -> NonNan {
    NonNan::new(
        (asteroid_loc.x as f64 - station_loc.x as f64)
            .atan2(asteroid_loc.y as f64 - station_loc.y as f64),
    )
}

/// Returns the order in which the asteroids in the map will be vaporized, given the station
/// location.
/// The station location is not vaporized.
fn plan_vaporization(
    map: &BTreeSet<AsteroidLocation>,
    station_loc: &AsteroidLocation,
) -> Vec<AsteroidLocation> {
    let mut map = map.clone();
    map.remove(station_loc);
    let mut order = Vec::new();

    // Computes the laser angle to the location
    let laser_angle = |dir: &AsteroidLocation| -> NonNan { compute_laser_angle(station_loc, dir) };

    let max_ang = NonNan(std::f64::consts::PI * 2.0); // the current angle from vertical
    let mut curr_angle = max_ang;

    while !map.is_empty() {
        let maybe_asteroid = map
            .iter()
            .cloned()
            .filter(|loc| laser_angle(loc) < curr_angle)
            .max_by_key(|loc| laser_angle(loc));

        if let Some(asteroid) = maybe_asteroid {
            let first = map.find_first(&station_loc, &asteroid).unwrap();
            order.push(first);
            map.remove(&first);
            curr_angle = laser_angle(&asteroid);
        } else {
            // if we didn't find an asteroid, then the map is either empty, or we don't have any
            // with a greater angle.  Set angle to less than -pi, the minimum value returned by atan2
            curr_angle = max_ang;
        }
    }
    order
}

fn string_to_asteroid_map(s: &str) -> BTreeSet<AsteroidLocation> {
    let mut map = BTreeSet::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(AsteroidLocation::new(x, y));
            }
        }
    }
    map
}

/// Calculates the position that can detect the most asteroids, and returns the position along
/// with the number of detected asteroids.
fn calculate_monitoring_station_position(
    map: &BTreeSet<AsteroidLocation>,
) -> (AsteroidLocation, usize) {
    let pos = map
        .iter()
        .max_by_key(|p| map.count_asteroids_visible_from(p))
        .unwrap();
    (*pos, map.count_asteroids_visible_from(pos))
}

fn get_asteroid_map() -> &'static str {
    "#.#.##..#.###...##.#....##....###\n\
     ...#..#.#.##.....#..##.#...###..#\n\
     ####...#..#.##...#.##..####..#.#.\n\
     ..#.#..#...#..####.##....#..####.\n\
     ....##...#.##...#.#.#...#.#..##..\n\
     .#....#.##.#.##......#..#..#..#..\n\
     .#.......#.....#.....#...###.....\n\
     #.#.#.##..#.#...###.#.###....#..#\n\
     #.#..........##..###.......#...##\n\
     #.#.........##...##.#.##..####..#\n\
     ###.#..#####...#..#.#...#..#.#...\n\
     .##.#.##.........####.#.#...##...\n\
     ..##...#..###.....#.#...#.#..#.##\n\
     .#...#.....#....##...##...###...#\n\
     ###...#..#....#............#.....\n\
     .#####.#......#.......#.#.##..#.#\n\
     #.#......#.#.#.#.......##..##..##\n\
     .#.##...##..#..##...##...##.....#\n\
     #.#...#.#.#.#.#..#...#...##...#.#\n\
     ##.#..#....#..##.#.#....#.##...##\n\
     ...###.#.#.......#.#..#..#...#.##\n\
     .....##......#.....#..###.....##.\n\
     ........##..#.#........##.......#\n\
     #.##.##...##..###.#....#....###.#\n\
     ..##.##....##.#..#.##..#.....#...\n\
     .#.#....##..###.#...##.#.#.#..#..\n\
     ..#..##.##.#.##....#...#.........\n\
     #...#.#.#....#.......#.#...#..#.#\n\
     ...###.##.#...#..#...##...##....#\n\
     ...#..#.#.#..#####...#.#...####.#\n\
     ##.#...#..##..#..###.#..........#\n\
     ..........#..##..#..###...#..#...\n\
     .#.##...#....##.....#.#...##...## "
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_asteroid_map() {
        let map = string_to_asteroid_map(
            ".#..#\n\
             .....\n\
             #####\n\
             ....#\n\
             ...##",
        );
        assert_eq!(10, map.len());
        assert!(map.contains(&(1, 0).into()));
        assert!(map.contains(&(0, 2).into()));
        assert!(map.contains(&(1, 2).into()));
        assert!(map.contains(&(2, 2).into()));
        assert!(map.contains(&(3, 2).into()));
        assert!(map.contains(&(4, 2).into()));

        assert!(!map.contains(&(0, 0).into()));
    }

    #[test]
    fn test_clear_path_exists() {
        let map = string_to_asteroid_map(
            ".#..#\n\
             .....\n\
             #####\n\
             ....#\n\
             ...##",
        );

        assert!(map.clear_path_exists(&(1, 0).into(), &(4, 0).into()));
        assert!(map.clear_path_exists(&(1, 0).into(), &(2, 3).into()));
        assert!(!map.clear_path_exists(&(1, 0).into(), &(3, 4).into()));
    }

    #[test]
    fn test_visibility_count() {
        let map = string_to_asteroid_map(
            ".#..#\n\
             .....\n\
             #####\n\
             ....#\n\
             ...##",
        );

        assert_eq!(8, map.count_asteroids_visible_from(&(3, 4).into()));
    }

    #[test]
    fn test_calculate_monitoring_station_position_large_example() {
        let map = string_to_asteroid_map(
            ".#..##.###...#######\n\
             ##.############..##.\n\
             .#.######.########.#\n\
             .###.#######.####.#.\n\
             #####.##.#.##.###.##\n\
             ..#####..#.#########\n\
             ####################\n\
             #.####....###.#.#.##\n\
             ##.#################\n\
             #####.##.###..####..\n\
             ..######..##.#######\n\
             ####.##.####...##..#\n\
             .#####..#.######.###\n\
             ##...#.##########...\n\
             #.##########.#######\n\
             .####.#.###.###.#.##\n\
             ....##.##.###..#####\n\
             .#.#.###########.###\n\
             #.#.#.#####.####.###\n\
             ###.##.####.##.#..##",
        );

        assert_eq!(
            (AsteroidLocation::new(11, 13), 210),
            calculate_monitoring_station_position(&map)
        );
    }

    #[test]
    fn test_calculate_monitoring_station_position_for_asteroid_map_day_10_part_1() {
        let map = string_to_asteroid_map(get_asteroid_map());
        assert_eq!(
            (AsteroidLocation::new(22, 28), 326),
            calculate_monitoring_station_position(&map)
        );
    }

    #[test]
    fn test_vaporization_order_test() {
        let map = string_to_asteroid_map(
            ".#....#####...#..\n\
             ##...##.#####..##\n\
             ##...#...#.#####.\n\
             ..#.....#...###..\n\
             ..#.#.....#....##",
        );
        let station_loc = AsteroidLocation::new(8, 3);
        let vaporization_order = plan_vaporization(&map, &station_loc);
        assert_eq!(vaporization_order.len(), map.len() - 1);
        assert_eq!(vaporization_order[0], AsteroidLocation::new(8, 1));
        assert_eq!(vaporization_order[1], AsteroidLocation::new(9, 0));
        assert_eq!(vaporization_order[2], AsteroidLocation::new(9, 1));
        assert_eq!(vaporization_order[3], AsteroidLocation::new(10, 0));
    }

    #[test]
    fn test_vaporization_order_answer() {
        let map = string_to_asteroid_map(get_asteroid_map());
        let (station, _) = calculate_monitoring_station_position(&map);
        let vaporization_order = plan_vaporization(&map, &station);
        assert_eq!(vaporization_order.len(), map.len() - 1);
        let vap_200 = vaporization_order[199];
        let ans = vap_200.x * 100 + vap_200.y;
        assert_eq!(1623, ans);
    }
}
