use std::collections::{HashSet};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY6: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let map: Map = input.parse().unwrap();

    let unique_tiles = map.count_guard_visited_tiles();
    println!("The guard visited {} unique tiles", unique_tiles);
}

fn puzzle2(input: &String) {
    let map: Map = input.parse().unwrap();

    let unique_tiles = map.count_obstructable_tiles_for_loops();
    println!("The guard can loop from {} unique tiles", unique_tiles);
}

#[derive(Eq, PartialEq, Copy, Clone, Default)]
enum Tile {
    #[default]
    Empty,
    Blocked,
    Guard
}

type Map = Grid<Tile>;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Trace {
    point: Point,
    direction: Directions,
}

impl Map {
    fn next_direction(guard_direction: &Directions) -> Directions {
        match guard_direction {
            Directions::Top => Directions::Right,
            Directions::Right => Directions::Bottom,
            Directions::Bottom => Directions::Left,
            Directions::Left => Directions::Top,
            _ => panic!("Guard moved in an unexpected direction: {:?}", guard_direction)
        }
    }

    fn count_guard_visited_tiles(&self) -> usize {
        // Find the guard, and from there:
        // - Move in the current direction (start UPwards) util not possible
        // - Turn 90deg right, and continue

        let guard_tile = self.entries().into_iter().find(|(_, t)| Tile::Guard.eq(t));
        if guard_tile.is_none() { return 0; }

        let mut guard_position = guard_tile.unwrap().0;
        let mut guard_direction = Directions::Top;

        // We might divert from the previously known path and end up in a random other loop
        let mut seen_tiles: HashSet<Point> = HashSet::new();

        loop {
            seen_tiles.insert(guard_position);

            let next_pos = guard_position.translate_in_direction(&guard_direction, 1);
            let next_tile = self.get(&next_pos);

            match next_tile {
                None => { break; } // Out of bounds, we're done.
                Some(Tile::Empty) | Some(Tile::Guard) => {
                    guard_position = next_pos;
                }
                Some(Tile::Blocked) => {
                    guard_direction = Self::next_direction(&guard_direction);
                }
            }
        }

        seen_tiles.len()
    }

    fn check_loop(map: &Self, from: &Point, direction: &Directions) -> bool {
        // To check a loop, we assume an obstacle was put in front of the guard (located at 'from' and moving in 'direction').
        // We let the guard walk until:
        // - It goes off the map (fail)
        // - It ends up at a spot (in the same direction) he's been before (success)
        let mut guard_position = *from;
        let mut guard_direction = Self::next_direction(direction);

        let mut blocked_map = map.clone();
        blocked_map.set(guard_position.translate_in_direction(direction, 1), Tile::Blocked);

        let mut loop_trace: HashSet<Trace> = HashSet::new();

        loop {
            let step_trace = Trace { point: guard_position, direction: guard_direction };
            if loop_trace.contains(&step_trace) { return true; } // we've been here before, so we're walking in a loop

            loop_trace.insert(step_trace);

            let next_pos = guard_position.translate_in_direction(&guard_direction, 1);
            let next_tile = blocked_map.get(&next_pos);

            match next_tile {
                None => { return false; } // Out of bounds, no loop
                Some(Tile::Empty) | Some(Tile::Guard) => {
                    guard_position = next_pos; // keep moving
                }
                Some(Tile::Blocked) => {
                    guard_direction = Self::next_direction(&guard_direction);
                }
            }
        }

    }

    fn count_obstructable_tiles_for_loops(&self) -> usize {
        // We need to figure out in how many tiles we can add an obstruction such that the guard will
        // go walk in a loop. Since the guard only turns right, we can determine if the guard ends up
        // at the same spot or not.

        let guard_tile = self.entries().into_iter().find(|(_, t)| Tile::Guard.eq(t));
        if guard_tile.is_none() { return 0; }

        let mut guard_position = guard_tile.unwrap().0;
        let mut guard_direction = Directions::Top;

        let mut seen_tiles: HashSet<Point> = HashSet::new();
        let mut obstuctions: HashSet<Point> = HashSet::new();

        loop {
            seen_tiles.insert(guard_position);

            let next_pos = guard_position.translate_in_direction(&guard_direction, 1);
            let next_tile = self.get(&next_pos);

            match next_tile {
                None => { break; } // Out of bounds, we're done.
                Some(Tile::Empty) | Some(Tile::Guard) => {
                    // If we _could_ insert an obstacle here, test it:
                    if !seen_tiles.contains(&next_pos) && !obstuctions.contains(&next_pos) && Self::check_loop(self, &guard_position, &guard_direction) {
                        obstuctions.insert(next_pos);
                    }

                    guard_position = next_pos;
                }
                Some(Tile::Blocked) => {
                    guard_direction = Self::next_direction(&guard_direction);
                }
            }
        }

        obstuctions.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day06::Map;

    const TEST_INPUT: &str = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...\n\
    ";

    #[test]
    fn test_count_guard_visited_tiles() {
        let map: Map = TEST_INPUT.parse().unwrap();

        assert_eq!(map.count_guard_visited_tiles(), 41);
    }

    #[test]
    fn test_count_obstructable_tiles_for_loops() {
        let map: Map = TEST_INPUT.parse().unwrap();

        assert_eq!(map.count_obstructable_tiles_for_loops(), 6);
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "#" => Ok(Tile::Blocked),
            "^" => Ok(Tile::Guard),
            _ => Err(format!("Unknown tile: {}", s))
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Empty => '.',
            Tile::Blocked => '#',
            Tile::Guard => '^'
        })
    }
}