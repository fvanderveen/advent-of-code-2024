use std::collections::VecDeque;
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::parser::Parser;

pub const DAY7: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let puzzle: Puzzle = input.parse().unwrap();

    println!("Sum of solvable equations: {}", puzzle.get_sum_of_solvable_equations(false));
}

fn puzzle2(input: &String) {
    let puzzle: Puzzle = input.parse().unwrap();

    println!("Sum of solvable equations with concatenation: {}", puzzle.get_sum_of_solvable_equations(true));
}

#[derive(Debug)]
struct Puzzle {
    equations: Vec<Equation>,
}

impl Puzzle {
    fn get_sum_of_solvable_equations(&self, with_concatenation: bool) -> usize {
        self.equations.iter().filter(|e| e.can_solve(with_concatenation)).map(|e| e.answer).sum()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Equation {
    answer: usize,
    parts: Vec<usize>,
}

impl Equation {
    fn concatenate(left: usize, right: usize) -> usize {
        let mut factor = 10;
        while factor < right { factor = factor * 10; }

        (left * factor) + right
    }

    fn can_solve(&self, with_concatenation: bool) -> bool {
        // Inserting either '+' or '*' between the parts, can we get to answer?

        let mut queue: VecDeque<Self> = VecDeque::new();
        queue.push_back(self.clone());

        while let Some(next) = queue.pop_front() {
            match &next.parts[..] {
                [] => {},
                [item] => {
                    return next.answer.eq(item);
                }
                [first, second] => {
                    // Two parts, if either adding or multiplying them ends up at answer, we're good
                    if (first + second) == next.answer {
                        return true;
                    }
                    if (first * second) == next.answer {
                        return true;
                    }
                    if with_concatenation && Self::concatenate(*first, *second) == next.answer {
                        return true;
                    }
                    // Otherwise, continue on with our lives.
                },
                [first, second, tail @ ..] => {
                    // If it becomes larger, makes no sense to continue
                    if (first + second) <= next.answer {
                        queue.push_back(Self { answer: next.answer, parts: tail.to_vec().prepend_item(&(first + second)) });
                    }
                    if (first * second) <= next.answer {
                        queue.push_back(Self { answer: next.answer, parts: tail.to_vec().prepend_item(&(first * second)) });
                    }

                    let concatenated = Self::concatenate(*first, *second);
                    if with_concatenation && concatenated <= next.answer {
                        queue.push_back(Self { answer: next.answer, parts: tail.to_vec().prepend_item(&concatenated) });
                    }
                },
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::{Equation, Puzzle};

    const TEST_INPUT: &str = "\
        190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20\n\
    ";

    #[test]
    fn test_equation_from_str() {
        let mut res: Result<Equation, _> = "190: 10 19".parse();
        assert_eq!(res, Ok(Equation { answer: 190, parts: vec![10, 19] }));

        res = "21037: 9 7 18 13".parse();
        assert_eq!(res, Ok(Equation { answer: 21037, parts: vec![9, 7, 18, 13] }));
    }

    #[test]
    fn test_can_solve() {
        assert_eq!(Equation { answer: 190, parts: vec![10, 19] }.can_solve(false), true);
        assert_eq!(Equation { answer: 156, parts: vec![15, 6] }.can_solve(false), false);
        assert_eq!(Equation { answer: 156, parts: vec![15, 6] }.can_solve(true), true);
    }

    #[test]
    fn test_get_sum_of_solvable_equations() {
        let puzzle: Puzzle = TEST_INPUT.parse().unwrap();

        assert_eq!(puzzle.get_sum_of_solvable_equations(false), 3749);
        assert_eq!(puzzle.get_sum_of_solvable_equations(true), 11387);
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(Equation::concatenate(15, 6), 156);
        assert_eq!(Equation::concatenate(42, 1337), 421337);
        assert_eq!(Equation::concatenate(1, 0), 10);
    }
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        let answer = parser.usize()?;
        parser.literal(":")?;

        let mut parts = vec![];
        while !parser.is_exhausted()
        {
            parts.push(parser.usize()?);
        }

        Ok(Self { answer, parts })
    }
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equations = s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;

        Ok(Self { equations })
    }
}