use num::Integer;
use std::collections::BTreeSet;
use std::convert::TryFrom;

/// Location of an asteroid.
/// `x`: The distance from the left edge
/// `y`: The distance from the top edge
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
        if start == end {
            return true;
        }
        let start: (i64, i64) = start.clone().into();
        let end: (i64, i64) = end.clone().into();

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
            if self.contains(&AsteroidLocation::try_from(curr).unwrap()) {
                return false;
            }
            curr = (curr.0 + slope.0, curr.1 + slope.1);
        }
        true
    }
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
}
