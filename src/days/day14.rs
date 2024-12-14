use std::io::stdin;
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Bounds, Grid, Point};
use crate::util::parser::Parser;

pub const DAY14: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let robots = parse_input(input).unwrap();

    println!("Safety factor at t=100: {}", get_safety_factor(&robots, 100, 101, 103));
}

fn puzzle2(input: &String) {
    let robots = parse_input(input).unwrap();

    // "find a Christmas tree"
    // We simply try to find a t where positions are unique, and then prompt the user to confirm
    // or deny the existence of a Christmas tree...

    let mut t = 1;
    loop {
        let points_at_t: Vec<_> = robots.iter().map(|r| r.position_after(t, 101, 103)).collect();
        let unique = points_at_t.deduplicate();
        if unique.len() == points_at_t.len() {
            println!("At {}", t);
            print_time(&robots, t, 101, 103);

            println!("Is there a tree? [Y/n]");
            let mut input =  String::new();
            stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "" | "Y" | "y" => return,
                _ => {}
            }
        }

        t += 1;
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Robot {
    start: Point,
    velocity_x: isize,
    velocity_y: isize,
}

impl Robot {
    fn position_after(&self, t: isize, width: isize, height: isize) -> Point {
        let raw_x = self.start.x + (t * self.velocity_x);
        let raw_y = self.start.y + (t * self.velocity_y);

        // After mod, we can have a negative number, so we add the length again. However, for a positive number that needs another mod.
        let wrapped_x = ((raw_x % width) + width) % width;
        let wrapped_y = ((raw_y % height) + height) % height;

        (wrapped_x, wrapped_y).into()
    }
}

fn parse_input(input: &str) -> Result<Vec<Robot>, String> {
    input.lines().map(|l| l.parse()).collect()
}

fn get_safety_factor(robots: &Vec<Robot>, t: isize, width: isize, height: isize) -> usize {
    let middle_width = width / 2;
    let middle_height = height / 2;

    let top_left = Bounds { top: 0, left: 0, width: middle_width as usize, height: middle_height as usize };
    let top_right = Bounds { top: 0, left: middle_width + 1, width: middle_width as usize, height: middle_height as usize };
    let bottom_left = Bounds { top: middle_height + 1, left: 0, width: middle_width as usize, height: middle_height as usize };
    let bottom_right = Bounds { top: middle_height + 1, left: middle_width + 1, width: middle_width as usize, height: middle_height as usize };

    let points_at_t: Vec<_> = robots.iter().map(|r| r.position_after(t, width, height)).collect();

    let top_left_robots = points_at_t.iter().filter(|p| top_left.contains(p)).count();
    let top_right_robots = points_at_t.iter().filter(|p| top_right.contains(p)).count();
    let bottom_left_robots = points_at_t.iter().filter(|p| bottom_left.contains(p)).count();
    let bottom_right_robots = points_at_t.iter().filter(|p| bottom_right.contains(p)).count();

    top_left_robots * top_right_robots * bottom_left_robots * bottom_right_robots
}

fn print_time(robots: &Vec<Robot>, t: isize, width: isize, height: isize) {
    // To compress the image on terminal somewhat, we fit a 2x2 square on on character (yay, unicode)
    let points_at_t: Vec<_> = robots.iter().map(|r| r.position_after(t, width, height)).collect();
    let mut grid = Grid::empty();

    for x in points_at_t {
        grid.set(x, true);
    }

    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            let tl = grid.get(&(x, y).into()).unwrap_or(false);
            let tr = grid.get(&(x + 1, y).into()).unwrap_or(false);
            let bl = grid.get(&(x, y + 1).into()).unwrap_or(false);
            let br = grid.get(&(x + 1, y + 1).into()).unwrap_or(false);

            match (tl, tr, bl, br) {
                (true, true, true, true) => print!("█"),
                (true, false, false, false) => print!("▘"),
                (false, true, false, false) => print!("▝"),
                (false, false, true, false) => print!("▖"),
                (false, false, false, true) => print!("▗"),
                (true, true, false, false) => print!("▀"),
                (false, false, true, true) => print!("▄"),
                (true, false, true, false) => print!("▌"),
                (false, true, false, true) => print!("▐"),
                (true, false, false, true) => print!("▚"),
                (false, true, true, false) => print!("▞"),
                (true, true, true, false) => print!("▛"),
                (true, true, false, true) => print!("▜"),
                (true, false, true, true) => print!("▙"),
                (false, true, true, true) => print!("▟"),
                (false, false, false, false) => print!(" "),
            }
        }

        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day14::{get_safety_factor, parse_input, Robot};

    const TEST_INPUT: &str = "\
        p=0,4 v=3,-3\n\
        p=6,3 v=-1,-3\n\
        p=10,3 v=-1,2\n\
        p=2,0 v=2,-1\n\
        p=0,0 v=1,3\n\
        p=3,0 v=-2,-2\n\
        p=7,6 v=-1,-3\n\
        p=3,0 v=-1,-2\n\
        p=9,3 v=2,3\n\
        p=7,3 v=-1,2\n\
        p=2,4 v=2,-3\n\
        p=9,5 v=-3,-3\n\
    ";

    #[test]
    fn test_position_after() {
        let robot = Robot { start: (2,4).into(), velocity_x: 2, velocity_y: -3 };
        assert_eq!(robot.position_after(0, 11, 7), (2,4).into());
        assert_eq!(robot.position_after(1, 11, 7), (4,1).into());
        assert_eq!(robot.position_after(2, 11, 7), (6,5).into());
        assert_eq!(robot.position_after(3, 11, 7), (8,2).into());
        assert_eq!(robot.position_after(4, 11, 7), (10,6).into());
        assert_eq!(robot.position_after(5, 11, 7), (1,3).into());
    }

    #[test]
    fn get_safety_factory() {
        let robots = parse_input(TEST_INPUT).unwrap();
        let result = get_safety_factor(&robots, 100, 11, 7);
        assert_eq!(result, 12);
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        parser.literal("p=")?;
        let start_x = parser.isize()?;
        parser.literal(",")?;
        let start_y = parser.isize()?;
        parser.literal("v=")?;
        let velocity_x = parser.isize()?;
        parser.literal(",")?;
        let velocity_y = parser.isize()?;

        Ok(Self { start: (start_x, start_y).into(), velocity_x, velocity_y })
    }
}