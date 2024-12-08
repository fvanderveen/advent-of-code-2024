use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Grid, Point};

pub const DAY8: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let map: Map = input.parse().unwrap();

    println!("There are {} antinodes on the map.", map.count_antinodes());
}

fn puzzle2(input: &String) {
    let map: Map = input.parse().unwrap();

    println!("There are {} (harmonic) antinodes on the map.", map.count_all_antinodes());
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
enum Tile {
    #[default]
    Void,
    Antenna(char),
}

type Map = Grid<Tile>;

impl Map {
    fn get_antennas(&self) -> HashMap<char, Vec<Point>> {
        let mut result = HashMap::new();

        for point in self.points() {
            let tile = self.get(&point);
            match tile {
                Some(Tile::Antenna(value)) => {
                    let list = result.get(&value).cloned().unwrap_or(vec![]);
                    result.insert(value, list.append_item(&point));
                }
                _ => {}
            }
        }

        result
    }

    fn get_antenna_pairs(&self) -> Vec<[Point; 2]> {
        let mut result = vec![];

        let antennas = self.get_antennas();
        for (_, points) in antennas {
            for i in 0..points.len() {
                for j in (i + 1)..points.len() {
                    result.push([points[i], points[j]]);
                }
            }
        }

        result
    }

    fn get_antinodes(left: &Point, right: &Point) -> [Point; 2] {
        let dx = right.x - left.x;
        let dy = right.y - left.y;

        let antinode_left: Point = (left.x - dx, left.y - dy).into();
        let antinode_right: Point = (right.x + dx, right.y + dy).into();

        [antinode_left, antinode_right]
    }

    fn get_all_antinodes(&self, left: &Point, right: &Point) -> Vec<Point> {
        let mut result = vec![];
        let dx = right.x - left.x;
        let dy = right.y - left.y;

        // Get all positions that fit in our bounds. Just go two ways and collect :shrug:
        let mut cur_x = left.x;
        let mut cur_y = left.y;
        while self.bounds.contains(&(cur_x, cur_y).into()) {
            result.push((cur_x, cur_y).into());
            cur_x = cur_x - dx;
            cur_y = cur_y - dy;
        }

        cur_x = right.x;
        cur_y = right.y;
        while self.bounds.contains(&(cur_x, cur_y).into()) {
            result.push((cur_x, cur_y).into());
            cur_x = cur_x + dx;
            cur_y = cur_y + dy;
        }

        result
    }

    fn count_antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();

        for [left, right] in self.get_antenna_pairs() {
            let [antinode_min, antinode_max] = Self::get_antinodes(&left, &right);

            if self.bounds.contains(&antinode_min) { antinodes.insert(antinode_min); }
            if self.bounds.contains(&antinode_max) { antinodes.insert(antinode_max); }
        }

        antinodes.len()
    }

    fn count_all_antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();

        for [left, right] in self.get_antenna_pairs() {
            for antinode in self.get_all_antinodes(&left, &right) {
                antinodes.insert(antinode);
            }
        }

        antinodes.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::Map;

    const TEST_INPUT: &str = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............\n\
    ";

    #[test]
    fn test_get_antennas() {
        let map: Map = TEST_INPUT.parse().unwrap();

        let antennas = map.get_antennas();

        let antennas_0 = antennas.get(&'0').unwrap();
        assert_eq!(antennas_0, &vec![(8, 1).into(), (5, 2).into(), (7, 3).into(), (4, 4).into()]);

        let antennas_a = antennas.get(&'A').unwrap();
        assert_eq!(antennas_a, &vec![(6, 5).into(),(8, 8).into(),(9, 9).into()]);
    }

    #[test]
    fn test_get_antenna_pairs() {
        let map: Map = TEST_INPUT.parse().unwrap();

        let pairs = map.get_antenna_pairs();

        assert_eq!(pairs.len(), 9);
        assert!(pairs.contains(&[(8, 1).into(), (5, 2).into()]));
        assert!(pairs.contains(&[(8, 1).into(), (7, 3).into()]));
        assert!(pairs.contains(&[(8, 1).into(), (4, 4).into()]));
        assert!(pairs.contains(&[(5, 2).into(), (7, 3).into()]));
        assert!(pairs.contains(&[(5, 2).into(), (4, 4).into()]));
        assert!(pairs.contains(&[(7, 3).into(), (4, 4).into()]));
        assert!(pairs.contains(&[(6, 5).into(), (8, 8).into()]));
        assert!(pairs.contains(&[(6, 5).into(), (9, 9).into()]));
        assert!(pairs.contains(&[(8, 8).into(), (9, 9).into()]));
    }

    #[test]
    fn test_count_antinodes() {
        let map: Map = TEST_INPUT.parse().unwrap();

        let antinodes = map.count_antinodes();
        assert_eq!(antinodes, 14);
    }

    #[test]
    fn test_get_antinodes() {
        assert_eq!(Map::get_antinodes(&(2, 2).into(), &(3, 2).into()), [(1,2).into(), (4,2).into()]);
        assert_eq!(Map::get_antinodes(&(8, 1).into(), &(5, 2).into()), [(11,0).into(), (2,3).into()]);
        assert_eq!(Map::get_antinodes(&(8, 2).into(), &(5, 1).into()), [(11,3).into(), (2,0).into()]);
    }

    #[test]
    fn test_get_all_antinodes() {
        let map: Map = TEST_INPUT.parse().unwrap();

        assert_eq!(map.get_all_antinodes(&(0,0).into(), &(1,2).into()), vec![(0,0).into(), (1,2).into(), (2,4).into(), (3,6).into(), (4,8).into(), (5,10).into()]);
    }

    #[test]
    fn test_count_all_antinodes() {
        let map: Map = TEST_INPUT.parse().unwrap();

        assert_eq!(map.count_all_antinodes(), 34);
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.chars().collect::<Vec<_>>()[..] {
            ['.'] => Ok(Tile::Void),
            [value] => Ok(Tile::Antenna(*value)),
            _ => Err(format!("Unknown tile: {}", s))
        }
    }
}