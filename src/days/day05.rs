use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;
use crate::days::Day;
use crate::util::number::parse_usize;
use crate::util::parser::Parser;

pub const DAY5: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let queue: PrintQueue = input.parse().unwrap();

    println!("Sum of middle-pages of valid updates: {}", queue.puzzle_1_solution());
}

fn puzzle2(input: &String) {
    let queue: PrintQueue = input.parse().unwrap();

    println!("Sum of middle-pages of fixed updates: {}", queue.puzzle_2_solution());
}

#[derive(PartialEq, Eq, Debug)]
struct PrintRule {
    first: usize,
    second: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct PrintQueue {
    rules: Vec<PrintRule>,
    updates: Vec<Vec<usize>>
}

impl PrintQueue {
    fn in_order(&self, update: &Vec<usize>) -> bool {
        for i in 0..update.len() {
            let current = &update[i];

            let head = &update[..i];
            let numbers_invalid_in_head = self.rules.iter().filter(|rule| rule.first.eq(current)).map(|rule| rule.second).collect::<HashSet<_>>();
            if head.iter().any(|v| numbers_invalid_in_head.contains(v)) { return false; }

            let tail = &update[i+1..];
            let numbers_invalid_in_tail = self.rules.iter().filter(|rule| rule.second.eq(current)).map(|rule| rule.first).collect::<HashSet<_>>();
            if tail.iter().any(|v| numbers_invalid_in_tail.contains(v)) { return false; }
        }

        true
    }

    fn sort_update(&self, update: &mut Vec<usize>) {
        // Can we build a sorting rule using the print rules? Would it perform?
        update.sort_by(|l, r| {
            // We should see if there is a rule with these two numbers. If so, we know the order; otherwise assume equal?
            let rule = self.rules.iter().find(|rule| (rule.first.eq(l) && rule.second.eq(r)) || (rule.first.eq(r) && rule.second.eq(l)));

            match rule {
                Some(rule) if rule.first.eq(l) => Ordering::Less,
                Some(rule) if rule.first.eq(r) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        });
    }

    fn puzzle_1_solution(&self) -> usize {
        // Get the sum of all middle pages in valid ordered updates.

        self.updates.iter().filter(|update| self.in_order(update)).map(|update| update[update.len() / 2]).sum()
    }

    fn puzzle_2_solution(&self) -> usize {
        let mut result = 0;

        for update in self.updates.iter().filter(|update| !self.in_order(update)) {
            let mut sorted = update.clone();
            self.sort_update(&mut sorted);
            result += sorted[sorted.len() / 2];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day05::{PrintQueue, PrintRule};

    const TEST_INPUT: &str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\n\
    ";

    #[test]
    fn test_parse() {
        let result: Result<PrintQueue, String> = TEST_INPUT.parse();

        assert!(result.is_ok());
        let queue = result.unwrap();

        assert_eq!(queue.rules.len(), 21);
        assert_eq!(queue.updates.len(), 6);

        assert_eq!(queue.rules[0], PrintRule { first: 47, second: 53 });
        assert_eq!(queue.updates[1], vec![97, 61, 53, 29, 13]);
    }

    #[test]
    fn test_in_order() {
        let queue: PrintQueue = TEST_INPUT.parse().unwrap();

        assert_eq!(queue.in_order(&queue.updates[0]), true);
        assert_eq!(queue.in_order(&queue.updates[1]), true);
        assert_eq!(queue.in_order(&queue.updates[2]), true);
        assert_eq!(queue.in_order(&queue.updates[3]), false);
        assert_eq!(queue.in_order(&queue.updates[4]), false);
        assert_eq!(queue.in_order(&queue.updates[5]), false);
    }

    #[test]
    fn test_puzzle_1() {
        let queue: PrintQueue = TEST_INPUT.parse().unwrap();

        assert_eq!(queue.puzzle_1_solution(), 143);
    }

    #[test]
    fn test_sort_update() {
        let queue: PrintQueue = TEST_INPUT.parse().unwrap();

        let mut update = vec![75,97,47,61,53];
        queue.sort_update(&mut update);
        assert_eq!(update, vec![97,75,47,61,53]);

        update = vec![61,13,29];
        queue.sort_update(&mut update);
        assert_eq!(update, vec![61,29,13]);

        update = vec![97,13,75,29,47];
        queue.sort_update(&mut update);
        assert_eq!(update, vec![97,75,47,29,13]);
    }

    #[test]
    fn test_puzzle_2() {
        let queue: PrintQueue = TEST_INPUT.parse().unwrap();

        assert_eq!(queue.puzzle_2_solution(), 123);
    }
}

impl FromStr for PrintRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);

        let first = parser.usize()?;
        parser.literal("|")?;
        let second = parser.usize()?;
        parser.ensure_exhausted()?;

        Ok(Self { first, second })
    }
}

impl FromStr for PrintQueue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<[_; 2], _> = s.split("\n\n").collect::<Vec<&str>>().try_into();
        match result {
            Ok([rules_input, updates_input]) => {
                let rules = rules_input.lines().map(|l| l.parse()).collect::<Result<Vec<_>, _>>()?;
                let updates = updates_input.lines().map(|l| l.split(",").map(|v| parse_usize(v)).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?;

                Ok(Self { rules, updates })
            }
            _ => {
                Err(format!("Could not parse print rules: {}", s))
            }
        }
    }
}