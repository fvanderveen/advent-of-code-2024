use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter, Write};
use crate::days::Day;
use crate::util::geometry::{Bounds, Directions, Grid, Point};

pub const DAY18: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let bytes = parse_input(input).unwrap();
    let mut grid = Grid::with_size(Bounds::from_size(71, 71));
    drop_bytes(&mut grid, &bytes[..1024]);
    let path = shortest_path_to_exit(&grid).unwrap();

    println!("After dropping 1024 bytes, the shortest path to exit is {} steps", path.len());
}

fn puzzle2(input: &String) {
    let bytes = parse_input(input).unwrap();
    let mut grid = Grid::with_size(Bounds::from_size(71, 71));
    drop_bytes(&mut grid, &bytes[..1024]);

    let blocker = find_byte_blocking_path(&mut grid, &bytes[1024..]).unwrap();
    println!("When {} dropped, the path got blocked!", blocker);
}

fn parse_input(input: &str) -> Result<Vec<Point>, String> {
    input.lines().map(|l| l.parse()).collect()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
enum State {
    #[default]
    Free,
    Corrupted
}

fn drop_bytes(grid: &mut Grid<State>, bytes: &[Point]) {
    for point in bytes {
        grid.set(*point, State::Corrupted);
    }
}

fn shortest_path_to_exit(grid: &Grid<State>) -> Option<Vec<Point>> {
    let mut queue: VecDeque<(Point, Vec<Point>)> = VecDeque::new();
    let mut visited = HashMap::new();

    let start = grid.bounds.top_left();
    let end = grid.bounds.bottom_right();

    queue.push_back((start, vec![]));

    while let Some((point, path)) = queue.pop_front() {
        if let Some(seen_len) = visited.get(&point) {
            if path.len().ge(seen_len) {
                continue; // Been here in less steps
            }
        }

        if point == end {
            return Some(path);
        }

        visited.insert(point, path.len());

        // We can move up, left, down, and right.
        for next_point in point.get_points_around(Directions::NonDiagonal) {
            match grid.get(&next_point) {
                None | Some(State::Corrupted) => continue, // Cannot move
                Some(State::Free) => {
                    let new_path = [path.clone(), vec![next_point]].concat();
                    queue.push_back((next_point, new_path));
                }
            }
        }
    }

    None
}

fn find_byte_blocking_path(grid: &mut Grid<State>, bytes: &[Point]) -> Option<Point> {
    // Start by getting the shortest path, as some bytes would already be dropped.
    // Then, drop bytes until one hits the path
    // Then try to find a new path.
    let mut path = match shortest_path_to_exit(grid) {
        Some(path) => path,
        None => return None
    };

    for byte in bytes {
        grid.set(*byte, State::Corrupted);

        if path.contains(byte) {
            path = match shortest_path_to_exit(grid) {
                Some(path) => path,
                None => return Some(*byte)
            };
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::days::day18::{drop_bytes, find_byte_blocking_path, parse_input, shortest_path_to_exit};
    use crate::util::geometry::{Bounds, Grid};

    #[test]
    fn test_drop_bytes() {
        let bytes = parse_input(TEST_INPUT).unwrap();
        let mut grid = Grid::with_size(Bounds::from_size(7, 7));
        drop_bytes(&mut grid, &bytes[..12]);

        assert_eq!(format!("{}", grid), "\
            ...#...\n\
            ..#..#.\n\
            ....#..\n\
            ...#..#\n\
            ..#..#.\n\
            .#..#..\n\
            #.#....\
        ");
    }

    #[test]
    fn test_shortest_path_to_exit() {
        let bytes = parse_input(TEST_INPUT).unwrap();
        let mut grid = Grid::with_size(Bounds::from_size(7, 7));
        drop_bytes(&mut grid, &bytes[..12]);

        let path = shortest_path_to_exit(&grid).unwrap();

        assert_eq!(path.len(), 22);
    }

    #[test]
    fn test_find_byte_blocking_path() {
        let bytes = parse_input(TEST_INPUT).unwrap();
        let mut grid = Grid::with_size(Bounds::from_size(7, 7));
        drop_bytes(&mut grid, &bytes[..12]);

        let result = find_byte_blocking_path(&mut grid, &bytes[12..]);
        assert_eq!(result, Some((6,1).into()));
    }

    const TEST_INPUT: &str = "\
        5,4\n\
        4,2\n\
        4,5\n\
        3,0\n\
        2,1\n\
        6,3\n\
        2,4\n\
        1,5\n\
        0,6\n\
        3,3\n\
        2,6\n\
        5,1\n\
        1,2\n\
        5,5\n\
        2,5\n\
        6,5\n\
        1,4\n\
        0,4\n\
        6,4\n\
        1,1\n\
        6,1\n\
        1,0\n\
        0,5\n\
        1,6\n\
        2,0\n\
    ";
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Free => f.write_char('.'),
            State::Corrupted => f.write_char('#'),
        }
    }
}