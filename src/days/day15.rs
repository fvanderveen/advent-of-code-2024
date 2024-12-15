use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY15: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let (mut grid, moves) = parse_input(input).unwrap();

    execute_moves(&mut grid, &moves);

    println!("Checksum of box GPS coords is: {}", get_gps_checksum(&grid));
}

fn puzzle2(input: &String) {
    let (grid, moves) = parse_input(input).unwrap();

    let mut widened_grid = widen_map(&grid);

    execute_moves(&mut widened_grid, &moves);

    println!("Checksum of box GPS coords is: {}", get_gps_checksum(&widened_grid));
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Move {
    Up,
    Right,
    Down,
    Left
}

fn parse_input(input: &str) -> Result<(Grid<Tile>, Vec<Move>), String> {
    let parts: [&str; 2] = input.split("\n\n").collect::<Vec<_>>().try_into().map_err(|_| "Invalid input parts".to_string())?;

    let grid: Grid<Tile> = parts[0].parse()?;
    let moves = parts[1].lines().flat_map(|l| l.chars().map(|c| c.to_string().parse::<Move>())).collect::<Result<Vec<_>, _>>()?;

    Ok((grid, moves))
}

fn widen_map(grid: &Grid<Tile>) -> Grid<Tile> {
    let mut new_grid = Grid::empty();

    for y in grid.bounds.y() {
        for x in grid.bounds.x() {
            match grid.get(&(x, y).into()) {
                Some(Tile::Wall) => {
                    new_grid.set((x * 2, y).into(), Tile::Wall);
                    new_grid.set(((x * 2) + 1, y).into(), Tile::Wall);
                },
                Some(Tile::Box) => {
                    new_grid.set((x * 2, y).into(), Tile::BoxLeft);
                    new_grid.set(((x * 2) + 1, y).into(), Tile::BoxRight);
                },
                Some(Tile::Robot) => {
                    new_grid.set((x * 2, y).into(), Tile::Robot);
                    new_grid.set(((x * 2) + 1, y).into(), Tile::Empty);
                },
                _ => {
                    // Should only be empty in the input
                    new_grid.set((x * 2, y).into(), Tile::Empty);
                    new_grid.set(((x * 2) + 1, y).into(), Tile::Empty);
                }
            }
        }
    }

    new_grid
}

fn execute_moves(grid: &mut Grid<Tile>, moves: &Vec<Move>) {
    // Take the robot tile, and move if possible according to the list of moves.
    // Boxes can be pushed, as long as there is an empty tile behind them.

    let mut robot_pos = grid.entries().iter().find(|(_, t)| Tile::Robot.eq(t)).map(|(p, _)| *p).unwrap(); // Just panic if the input is wrong.

    for mov in moves {
        if can_move(grid, &robot_pos, mov) {
            robot_pos = do_move(grid, &robot_pos, mov, 0);
        }
    }
}

fn can_move(grid: &Grid<Tile>, pos: &Point, mov: &Move) -> bool {
    match grid.get(pos) {
        Some(Tile::Box | Tile::Robot | Tile::BoxLeft | Tile::BoxRight) => {},
        _ => return false
    };

    if let Some((next_pos, next_tile)) = mov.get_tile_from(grid, pos) {
        match next_tile {
            Tile::Empty => true,
            Tile::Box => can_move(grid, &next_pos, mov),
            Tile::BoxRight => {
                // If we're moving right, we only need to check this half.
                if Move::Right.eq(mov) {
                    return can_move(grid, &next_pos, mov);
                }

                // If we're moving left, we only need to check the left half.
                if Move::Left.ne(mov) && !can_move(grid, &next_pos, mov) {
                    return false;
                }

                // Get the left part, and see if _both_ can move
                let left_pos = next_pos.translate_in_direction(&Directions::Left, 1);
                match grid.get(&left_pos) {
                    Some(Tile::BoxLeft) => can_move(grid, &left_pos, mov),
                    _ => false
                }
            },
            Tile::BoxLeft => {
                // If we're moving left, we only need to check this half.
                if Move::Left.eq(mov) {
                    return can_move(grid, &next_pos, mov);
                }

                // If we're moving right, we only need to check the left half.
                if Move::Right.ne(mov) && !can_move(grid, &next_pos, mov) {
                    return false;
                }

                // Get the right part, and see if _both_ can move
                let right_pos = next_pos.translate_in_direction(&Directions::Right, 1);
                match grid.get(&right_pos) {
                    Some(Tile::BoxRight) => can_move(grid, &right_pos, mov),
                    _ => false
                }
            }
            _ => false
        }
    } else {
        false
    }
}

fn do_move(grid: &mut Grid<Tile>, pos: &Point, mov: &Move, safety: usize) -> Point {
    let tile = match grid.get(pos) {
        Some(t @ (Tile::Empty | Tile::Box | Tile::Robot | Tile::BoxRight | Tile::BoxLeft)) => t,
        v => panic!("Can't move tile {:?}", v)
    };

    let new_pos = mov.translate(pos);

    match tile {
        Tile::Empty => { /* nothing to do */ },
        t @ (Tile::Box | Tile::Robot) => {
            do_move(grid, &new_pos, mov, safety + 1);
            grid.set(new_pos, t);
            grid.set(*pos, Tile::Empty);
        },
        Tile::BoxLeft => {
            do_move(grid, &new_pos, mov, safety + 1);
            grid.set(new_pos, Tile::BoxLeft);
            grid.set(*pos, Tile::Empty);

            // Note: only when moving up or down we need to explicitly move the other half along.
            if Move::Up.eq(mov) || Move::Down.eq(mov) {
                let right_pos = new_pos.translate_in_direction(&Directions::Right, 1);
                do_move(grid, &right_pos, mov, safety + 1);
                grid.set(right_pos, Tile::BoxRight);
                grid.set(pos.translate_in_direction(&Directions::Right, 1), Tile::Empty);
            }
        },
        Tile::BoxRight => {
            do_move(grid, &new_pos, mov, safety + 1);
            grid.set(new_pos, Tile::BoxRight);
            grid.set(*pos, Tile::Empty);

            // Note: only when moving up or down we need to explicitly move the other half along.
            if Move::Up.eq(mov) || Move::Down.eq(mov) {
                let left_pos = new_pos.translate_in_direction(&Directions::Left, 1);
                do_move(grid, &left_pos, mov, safety + 1);
                grid.set(left_pos, Tile::BoxLeft);
                grid.set(pos.translate_in_direction(&Directions::Left, 1), Tile::Empty);
            }
        },
        Tile::Wall => panic!()
    }

    new_pos
}

fn get_gps_checksum(grid: &Grid<Tile>) -> usize {
    // GPS is determined by taking the distance from top edge * 100, adding distance from left edge * 1.
    let mut result=  0;

    for (p, t) in grid.entries() {
        match t {
            Tile::Box | Tile::BoxLeft => {
                result += (p.y * 100 + p.x) as usize;
            },
            _ => {}
        }
    }

    result
}

impl Move {
    fn translate(&self, pos: &Point) -> Point {
        match self {
            Move::Up => pos.translate_in_direction(&Directions::Top, 1),
            Move::Right => pos.translate_in_direction(&Directions::Right, 1),
            Move::Down => pos.translate_in_direction(&Directions::Bottom, 1),
            Move::Left => pos.translate_in_direction(&Directions::Left, 1),
        }
    }

    fn get_tile_from(&self, grid: &Grid<Tile>, pos: &Point) -> Option<(Point, Tile)> {
        let next_point = self.translate(pos);

        grid.get(&next_point).map(|t| (next_point, t))
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day15::{execute_moves, get_gps_checksum, parse_input, widen_map};

    const SMALL_INPUT: &str = "\
        ########\n\
        #..O.O.#\n\
        ##@.O..#\n\
        #...O..#\n\
        #.#.O..#\n\
        #...O..#\n\
        #......#\n\
        ########\n\
        \n\
        <^^>>>vv<v>>v<<\n\
    ";

    const TEST_INPUT: &str = "\
        ##########\n\
        #..O..O.O#\n\
        #......O.#\n\
        #.OO..O.O#\n\
        #..O@..O.#\n\
        #O#..O...#\n\
        #O..O..O.#\n\
        #.OO.O.OO#\n\
        #....O...#\n\
        ##########\n\
        \n\
        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
    ";

    const SMALL_WIDEN_TEST: &str = "\
        #######\n\
        #...#.#\n\
        #.....#\n\
        #..OO@#\n\
        #..O..#\n\
        #.....#\n\
        #######\n\
        \n\
        <vv<<^^<<^^\n\
    ";

    #[test]
    fn test_small_execute_moves() {
        let (mut grid, moves) = parse_input(SMALL_INPUT).unwrap();

        execute_moves(&mut grid, &moves);

        assert_eq!(format!("{}", grid), "\
            ########\n\
            #....OO#\n\
            ##.....#\n\
            #.....O#\n\
            #.#O@..#\n\
            #...O..#\n\
            #...O..#\n\
            ########\
        ");
    }

    #[test]
    fn test_execute_moves() {
        let (mut grid, moves) = parse_input(TEST_INPUT).unwrap();

        execute_moves(&mut grid, &moves);

        assert_eq!(format!("{}", grid), "\
            ##########\n\
            #.O.O.OOO#\n\
            #........#\n\
            #OO......#\n\
            #OO@.....#\n\
            #O#.....O#\n\
            #O.....OO#\n\
            #O.....OO#\n\
            #OO....OO#\n\
            ##########\
        ");
    }

    #[test]
    fn test_get_gps_checksum() {
        let (mut small_grid, small_moves) = parse_input(SMALL_INPUT).unwrap();
        execute_moves(&mut small_grid, &small_moves);
        assert_eq!(get_gps_checksum(&small_grid), 2028);

        let (mut large_grid, large_moves) = parse_input(TEST_INPUT).unwrap();
        execute_moves(&mut large_grid, &large_moves);
        assert_eq!(get_gps_checksum(&large_grid), 10092);
    }

    #[test]
    fn test_widen_map() {
        let (small_grid, _) = parse_input(SMALL_WIDEN_TEST).unwrap();

        let small_wider_grid = widen_map(&small_grid);

        assert_eq!(format!("{}", small_wider_grid), "\
            ##############\n\
            ##......##..##\n\
            ##..........##\n\
            ##....[][]@.##\n\
            ##....[]....##\n\
            ##..........##\n\
            ##############\
        ");
    }

    #[test]
    fn test_move_widen_map() {
        let (small_grid, moves) = parse_input(SMALL_WIDEN_TEST).unwrap();

        let mut small_wider_grid = widen_map(&small_grid);
        execute_moves(&mut small_wider_grid, &moves);

        assert_eq!(format!("{}", small_wider_grid), "\
            ##############\n\
            ##...[].##..##\n\
            ##...@.[]...##\n\
            ##....[]....##\n\
            ##..........##\n\
            ##..........##\n\
            ##############\
        ");

        let (large_grid, moves) = parse_input(TEST_INPUT).unwrap();

        let mut large_wider_grid = widen_map(&large_grid);
        execute_moves(&mut large_wider_grid, &moves);

        assert_eq!(format!("{}", large_wider_grid), "\
            ####################\n\
            ##[].......[].[][]##\n\
            ##[]...........[].##\n\
            ##[]........[][][]##\n\
            ##[]......[]....[]##\n\
            ##..##......[]....##\n\
            ##..[]............##\n\
            ##..@......[].[][]##\n\
            ##......[][]..[]..##\n\
            ####################\
        ");
    }

    #[test]
    fn test_widened_gps() {
        let (large_grid, large_moves) = parse_input(TEST_INPUT).unwrap();
        let mut widened_grid = widen_map(&large_grid);
        execute_moves(&mut widened_grid, &large_moves);
        assert_eq!(get_gps_checksum(&widened_grid), 9021);
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "#" => Ok(Tile::Wall),
            "O" => Ok(Tile::Box),
            "@" => Ok(Tile::Robot),
            _ => Err(format!("Unknown tile: {}", s)),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::Wall => f.write_char('#'),
            Tile::Box => f.write_char('O'),
            Tile::Robot => f.write_char('@'),
            Tile::BoxLeft => f.write_char('['),
            Tile::BoxRight => f.write_char(']'),
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Move::Up),
            ">" => Ok(Move::Right),
            "v" => Ok(Move::Down),
            "<" => Ok(Move::Left),
            _ => Err(format!("Unknown move: {}", s)),
        }
    }
}