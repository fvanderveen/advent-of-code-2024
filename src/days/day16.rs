use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY16: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let maze: Maze = input.parse().unwrap();

    println!("Our maze's lowest score is {}", maze.solve().unwrap());
}

fn puzzle2(input: &String) {
    let maze: Maze = input.parse().unwrap();

    println!("Our maze has {} tiles best to sit at", maze.get_best_tiles_count().unwrap());
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Start,
    End
}

type Maze = Grid<Tile>;

#[derive(Eq, PartialEq, Clone, Debug)]
struct SolveMazeEntry {
    tile: Point,
    direction: Directions,
    current_points: usize,
    path: HashSet<Point>,
}

impl Maze {
    fn get_left_direction(directions: Directions) -> Directions {
        match directions {
            Directions::Top => Directions::Left,
            Directions::Right => Directions::Top,
            Directions::Bottom => Directions::Right,
            Directions::Left => Directions::Bottom,
            _ => unreachable!()
        }
    }

    fn get_right_direction(directions: Directions) -> Directions {
        match directions {
            Directions::Top => Directions::Right,
            Directions::Right => Directions::Bottom,
            Directions::Bottom => Directions::Left,
            Directions::Left => Directions::Top,
            _ => unreachable!()
        }
    }

    fn solve(&self) -> Option<usize> {
        self.solve_internal().map(|(v, _)| v)
    }

    fn get_best_tiles_count(&self) -> Option<usize> {
        self.solve_internal().map(|(_, v)| v.len())
    }

    fn solve_internal(&self) -> Option<(usize, HashSet<Point>)> {
        // Rotating clock/counter-clockwise would cost 1000 points
        // Moving forward costs 1 point.
        // I assume this would work through Dĳkstra, though we need some logic to detect the turn taken

        // First, find the start position.
        let start_pos = self.entries().iter().find(|(_, t)| Tile::Start.eq(t)).map(|(p, _)| *p)?;

        // Find our goal:
        let end_pos = self.entries().iter().find(|(_, t)| Tile::End.eq(t)).map(|(p, _)| *p)?;

        // Run the algo
        let mut queue: BinaryHeap<SolveMazeEntry> = BinaryHeap::new();
        let mut visited: HashMap<(Point, Directions), usize> = HashMap::new();

        let mut best_score = None;
        let mut best_path_points = HashSet::new();

        queue.push(SolveMazeEntry { tile: start_pos, direction: Directions::Right, current_points: 0, path: HashSet::from([start_pos]) });

        while let Some(current) = queue.pop() {
            if let Some(seen_value) = visited.get(&(current.tile, current.direction)) {
                if current.current_points.gt(seen_value) {
                    // Been here with a same or better score already, no need to pursue.
                    continue;
                }

                // Note: we do re-walk when getting with the same value just to grab all possible shortest routes.
            }

            visited.insert((current.tile, current.direction), current.current_points);

            if current.tile == end_pos {
                if let Some(previous_best) = best_score {
                    if previous_best < current.current_points {
                        // Due to re-running the path, we can end up at end from a different direction.
                        // Since we use shortest-path at the core; if we _have_ a score here, it has to be
                        // the best already.
                        continue;
                    }
                }

                // We're done!
                best_score = Some(current.current_points);
                for path_point in current.path {
                    best_path_points.insert(path_point);
                }
                continue; // traversing other shortest paths.
            }

            // Get next options
            // Forward
            let forward_tile = current.tile.translate_in_direction(&current.direction, 1);
            if let Some(Tile::Empty | Tile::End) = self.get(&forward_tile) {
                let mut path = current.path.clone();
                path.insert(forward_tile);
                queue.push(SolveMazeEntry { tile: forward_tile, direction: current.direction, current_points: current.current_points + 1, path });
            }

            // Left
            let left_dir = Self::get_left_direction(current.direction);
            let left_tile = current.tile.translate_in_direction(&left_dir, 1);
            if let Some(Tile::Empty | Tile::End) = self.get(&left_tile) {
                let mut path = current.path.clone();
                path.insert(left_tile);
                queue.push(SolveMazeEntry { tile: left_tile, direction: left_dir, current_points: current.current_points + 1001, path });
            }

            // Right
            let right_dir = Self::get_right_direction(current.direction);
            let right_tile = current.tile.translate_in_direction(&right_dir, 1);
            if let Some(Tile::Empty | Tile::End) = self.get(&right_tile) {
                let mut path = current.path.clone();
                path.insert(right_tile);
                queue.push(SolveMazeEntry { tile: right_tile, direction: right_dir, current_points: current.current_points + 1001, path });
            }
        }

        best_score.map(|p| (p, best_path_points))
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day16::Maze;

    const EXAMPLE_MAZE_A: &str = "\
        ###############\n\
        #.......#....E#\n\
        #.#.###.#.###.#\n\
        #.....#.#...#.#\n\
        #.###.#####.#.#\n\
        #.#.#.......#.#\n\
        #.#.#####.###.#\n\
        #...........#.#\n\
        ###.#.#####.#.#\n\
        #...#.....#.#.#\n\
        #.#.#.###.#.#.#\n\
        #.....#...#.#.#\n\
        #.###.#.#.#.#.#\n\
        #S..#.....#...#\n\
        ###############\n\
    ";

    const EXAMPLE_MAZE_B: &str = "\
        #################\n\
        #...#...#...#..E#\n\
        #.#.#.#.#.#.#.#.#\n\
        #.#.#.#...#...#.#\n\
        #.#.#.#.###.#.#.#\n\
        #...#.#.#.....#.#\n\
        #.#.#.#.#.#####.#\n\
        #.#...#.#.#.....#\n\
        #.#.#####.#.###.#\n\
        #.#.#.......#...#\n\
        #.#.###.#####.###\n\
        #.#.#...#.....#.#\n\
        #.#.#.#####.###.#\n\
        #.#.#.........#.#\n\
        #.#.#.#########.#\n\
        #S#.............#\n\
        #################\n\
    ";

    #[test]
    fn test_solve() {
        let maze_a: Maze = EXAMPLE_MAZE_A.parse().unwrap();
        let maze_b: Maze = EXAMPLE_MAZE_B.parse().unwrap();

        assert_eq!(maze_a.solve(), Some(7036));
        assert_eq!(maze_b.solve(), Some(11048));
    }

    #[test]
    fn test_get_best_tiles_count() {
        let maze_a: Maze = EXAMPLE_MAZE_A.parse().unwrap();
        let maze_b: Maze = EXAMPLE_MAZE_B.parse().unwrap();

        assert_eq!(maze_a.get_best_tiles_count(), Some(45));
        assert_eq!(maze_b.get_best_tiles_count(), Some(64));
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "#" => Ok(Tile::Wall),
            "S" => Ok(Tile::Start),
            "E" => Ok(Tile::End),
            _ => Err(format!("Unknown tile: {}", s))
        }
    }
}

impl Ord for SolveMazeEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // To make BinaryHeap.pop work correctly, the _smaller_ score needs to come out as Greater
        // As such, we compare in the reverse order
        other.current_points.cmp(&self.current_points)
    }
}

impl PartialOrd for SolveMazeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
