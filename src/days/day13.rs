use std::cmp::Ordering;
use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::Point;
use crate::util::parser::Parser;

pub const DAY13: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let machines = parse_input(input).unwrap();

    let cost: usize = machines.iter().filter_map(|m| m.get_corrected_solve(0)).map(|s| s.cost()).sum();
    println!("It takes {} coins to win all possible prizes.", cost);
}

fn puzzle2(input: &String) {
    let machines = parse_input(input).unwrap();

    let cost: usize = machines.iter().filter_map(|m| m.get_corrected_solve(10_000_000_000_000)).map(|s| s.cost()).sum();
    println!("Oops! It takes {} coins to really win all possible prizes.", cost);
}

fn parse_input(input: &str) -> Result<Vec<ClawMachine>, String> {
    input.split("\n\n").map(|p| p.parse()).collect()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct ClawMachine {
    prize_loc: Point,
    button_a: Point,
    button_b: Point,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct ClawMachineState {
    a_presses: usize,
    b_presses: usize,
}

impl ClawMachineState {
    fn cost(&self) -> usize {
        self.a_presses * 3 + self.b_presses
    }
}

impl ClawMachine {
    fn get_corrected_solve(&self, correction: isize) -> Option<ClawMachineState> {
        // Add 10_000_000_000_000 to prize x and y, try to solve.
        // Shortest path definitely won't work for that, too many options.
        // Solving with Cramer's rule

        fn det(a: isize, b: isize, c: isize, d: isize) -> isize {
            // calculate
            // |a c|
            // |b d|
            a * d - b * c
        }

        let corrected_loc_x = self.prize_loc.x + correction;
        let corrected_loc_y = self.prize_loc.y + correction;

        let buttons_det = det(self.button_a.x, self.button_a.y, self.button_b.x, self.button_b.y);
        if buttons_det == 0 {
            return None;
        }

        let button_a_det = det(corrected_loc_x, corrected_loc_y, self.button_b.x, self.button_b.y);
        if button_a_det % buttons_det != 0 { return None } // Correct for integer rounding

        let a_presses = button_a_det / buttons_det;

        let button_b_det = det(self.button_a.x, self.button_a.y, corrected_loc_x, corrected_loc_y);
        if button_b_det % buttons_det != 0 { return None }

        let b_presses = button_b_det / buttons_det;

        Some(ClawMachineState { a_presses: a_presses as usize, b_presses: b_presses as usize })
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::days::day13::{parse_input, ClawMachine, ClawMachineState};

    const TEST_INPUT: &str = "\
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());

        let machines = result.unwrap();
        assert_eq!(machines.len(), 4);
        assert_eq!(machines[0], ClawMachine {
            prize_loc: (8400, 5400).into(),
            button_a: (94, 34).into(),
            button_b: (22, 67).into(),
        })
    }

    #[test]
    fn test_get_corrected_solve() {
        let machines = parse_input(TEST_INPUT).unwrap();

        assert_eq!(machines[0].get_corrected_solve(0), Some(ClawMachineState { a_presses: 80, b_presses: 40 }));
        assert_eq!(machines[1].get_corrected_solve(0), None);
        assert_eq!(machines[2].get_corrected_solve(0), Some(ClawMachineState { a_presses: 38, b_presses: 86 }));
        assert_eq!(machines[3].get_corrected_solve(0), None);

        assert_eq!(machines[0].get_corrected_solve(10_000_000_000_000), None);
        assert_eq!(machines[1].get_corrected_solve(10_000_000_000_000), Some(ClawMachineState { a_presses: 118679050709, b_presses: 103199174542 }));
        assert_eq!(machines[2].get_corrected_solve(10_000_000_000_000), None);
        assert_eq!(machines[3].get_corrected_solve(10_000_000_000_000), Some(ClawMachineState { a_presses: 102851800151, b_presses: 107526881786 }));
    }

    #[test]
    fn test_state_ord() {
        let state_a = ClawMachineState { a_presses: 0, b_presses: 10 };
        let state_b = ClawMachineState { a_presses: 4, b_presses: 0 };
        let state_c = ClawMachineState { a_presses: 2, b_presses: 3 };
        let state_d = ClawMachineState { a_presses: 2, b_presses: 4 };

        assert_eq!(state_a.cmp(&state_b), Ordering::Greater); // A costs less, so should be on top of the heap for shortest path
        assert_eq!(state_b.cmp(&state_c), Ordering::Less);
        assert_eq!(state_b.cmp(&state_d), Ordering::Less);
        assert_eq!(state_c.cmp(&state_d), Ordering::Greater);
        assert_eq!(state_a.cmp(&state_d), Ordering::Equal);
    }
}

impl FromStr for ClawMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        parser.literal("Button A:")?;
        parser.literal("X+")?;
        let button_a_x = parser.isize()?;
        parser.literal(",")?;
        parser.literal("Y+")?;
        let button_a_y = parser.isize()?;

        parser.literal("Button B:")?;
        parser.literal("X+")?;
        let button_b_x = parser.isize()?;
        parser.literal(",")?;
        parser.literal("Y+")?;
        let button_b_y = parser.isize()?;

        parser.literal("Prize:")?;
        parser.literal("X=")?;
        let prize_x = parser.isize()?;
        parser.literal(",")?;
        parser.literal("Y=")?;
        let prize_y = parser.isize()?;

        Ok(Self {
            prize_loc: Point { x: prize_x, y: prize_y },
            button_a: Point { x: button_a_x, y: button_a_y },
            button_b: Point { x: button_b_x, y: button_b_y },
        })
    }
}

impl Ord for ClawMachineState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost().cmp(&other.cost()).reverse()
    }
}

impl PartialOrd<Self> for ClawMachineState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
