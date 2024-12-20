use std::collections::HashMap;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY20: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let maze: Maze = input.parse().unwrap();
    let cheats =  maze.find_cheats(2);

    let good_cheats = cheats.iter().filter(|c| c.time_saved >= 100).count();
    println!("There are {} cheats that save more than 100ps", good_cheats);
}

fn puzzle2(input: &String) {
    let maze: Maze = input.parse().unwrap();
    let cheats =  maze.find_cheats(20);

    let better_cheats = cheats.iter().filter(|c| c.time_saved >= 100).count();
    println!("There are {} better cheats that save more than 100ps", better_cheats);
}

#[derive(Eq, PartialEq, Copy, Clone, Default, Debug)]
enum Tile {
    #[default]
    Empty,
    Start,
    End,
    Wall
}

type Maze = Grid<Tile>;

struct Cheat {
    time_saved: usize,
}

impl Maze {
    fn find_cheats(&self, max_cheat_length: usize) -> Vec<Cheat> {
        // Battle plan:
        // - Fill map with 'shortest' path values (There is just a single track, given puzzle description)
        // - For every point(N), figure out if we can cheat:
        //   - Move two points in every direction to point(M)
        //   - Check if N+2 is smaller than M
        //   - If so, result in a cheat with time_saved = M - N+2

        let mut distance_map = HashMap::new();
        let start_point = self.entries().iter().find(|(_, t)| Tile::Start.eq(t)).map(|(p, _)| *p).unwrap();
        let end_point = self.entries().iter().find(|(_, t)| Tile::End.eq(t)).map(|(p, _)| *p).unwrap();

        // loop 1, fill distance_map
        let mut current = start_point;
        let mut distance = 0;
        let mut path = vec![];

        loop {
            path.push(current);
            distance_map.insert(current, distance);

            if current == end_point { break; }

            // There should always be a single point around that is Empty (or the End) and not visited, since there is just one path :shrug:
            distance += 1;
            current = current.get_points_around(Directions::NonDiagonal).iter().find(|p| !distance_map.contains_key(p) && (self.get(p) == Some(Tile::Empty) || self.get(p) == Some(Tile::End))).copied().unwrap();
        }

        // loop 2, check for cheats
        fn get_cheats_around(point: &Point, max_cheat_length: usize, distances: &HashMap<Point, usize>) -> Vec<Cheat> {
            // Get all points within 'max_cheat_length' manhattan distance of point.
            // Find all locations with a better score than that of point + 2
            // Yield the cheats!
            let start_value = match distances.get(point) {
                Some(value) => *value,
                None => return vec![]
            };

            point.get_points_within_manhattan_distance(max_cheat_length).iter().filter_map(|p| {
                if let Some(distance) = distances.get(p) {
                    let hack_length = point.manhattan_distance(p) as usize;
                    if (start_value + hack_length).lt(distance) {
                        // We got a cheat!
                        return Some(Cheat { time_saved: distance - start_value - hack_length });
                    }
                }

                None
            }).collect()
        }

        path.iter().flat_map(|p| get_cheats_around(p, max_cheat_length, &distance_map)).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::days::day20::{Cheat, Maze};

    const TEST_INPUT: &str = "\
        ###############\n\
        #...#...#.....#\n\
        #.#.#.#.#.###.#\n\
        #S#...#.#.#...#\n\
        #######.#.#.###\n\
        #######.#.#...#\n\
        #######.#.###.#\n\
        ###..E#...#...#\n\
        ###.#######.###\n\
        #...###...#...#\n\
        #.#####.#.###.#\n\
        #.#...#.#.#...#\n\
        #.#.#.#.#.#.###\n\
        #...#...#...###\n\
        ###############\n\
    ";

    #[test]
    fn test_find_cheats() {
        let maze: Maze = TEST_INPUT.parse().unwrap();
        let cheats = maze.find_cheats(2);

        assert_eq!(cheats.len(), 44);
        let cheats_by_length = cheats.into_iter().fold(HashMap::new(), |mut map: HashMap<usize, Vec<Cheat>>, c| {
            match map.get_mut(&c.time_saved) {
                Some(list) => list.push(c),
                None => { map.insert(c.time_saved, vec![c]); }
            }

            map
        });

        assert_eq!(cheats_by_length.get(&2).map(|l| l.len()), Some(14));
        assert_eq!(cheats_by_length.get(&4).map(|l| l.len()), Some(14));
        assert_eq!(cheats_by_length.get(&6).map(|l| l.len()), Some(2));
        assert_eq!(cheats_by_length.get(&8).map(|l| l.len()), Some(4));
        assert_eq!(cheats_by_length.get(&10).map(|l| l.len()), Some(2));
        assert_eq!(cheats_by_length.get(&12).map(|l| l.len()), Some(3));
        assert_eq!(cheats_by_length.get(&20).map(|l| l.len()), Some(1));
        assert_eq!(cheats_by_length.get(&36).map(|l| l.len()), Some(1));
        assert_eq!(cheats_by_length.get(&38).map(|l| l.len()), Some(1));
        assert_eq!(cheats_by_length.get(&40).map(|l| l.len()), Some(1));
        assert_eq!(cheats_by_length.get(&64).map(|l| l.len()), Some(1));

        let better_cheats = maze.find_cheats(20).iter().filter(|c| c.time_saved >= 50).count();
        assert_eq!(better_cheats, 285);
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Wall),
            "S" => Ok(Self::Start),
            "E" => Ok(Self::End),
            _ => Err(format!("Unknown tile: {}", s))
        }
    }
}