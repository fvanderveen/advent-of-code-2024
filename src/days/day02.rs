use std::str::FromStr;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY2: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let reports = parse_input(input).unwrap();

    let safe_count = reports.iter().filter(|r| r.is_safe()).count();
    println!("Of all reports, {} are safe.", safe_count);
}

fn puzzle2(input: &String) {
    let reports = parse_input(input).unwrap();

    let safe_count = reports.iter().filter(|r| r.is_safe_dampening()).count();
    println!("Of all reports, {} are safe with dampening.", safe_count);
}

fn parse_input(input: &str) -> Result<Vec<Report>, String> {
    input.lines().map(|l| l.parse()).collect()
}

#[derive(Eq, PartialEq, Debug)]
struct Report {
    levels: Vec<usize>
}

impl Report {
    fn _is_safe_internal(&self, ignore_idx: Option<usize>) -> bool {
        let mut levels = self.levels.clone();
        if let Some(idx) = ignore_idx {
            levels.remove(idx);
        }

        for idx in 1..levels.len() {
            if idx > 1 {
                let left = levels[idx - 2] < levels[idx - 1];
                let right = levels[idx - 1] < levels[idx];
                if left != right { return false }
            }

            if !(1..=3).contains(&levels[idx-1].abs_diff(levels[idx])) { return false; }
        }

        true
    }

    fn is_safe(&self) -> bool {
        // The levels are either all increasing or all decreasing.
        // Any two adjacent levels differ by at least one and at most three.

        self._is_safe_internal(None)
    }

    fn is_safe_dampening(&self) -> bool {
        // Same as above, but allows ignoring _a single value_.

        if self._is_safe_internal(None) { return true }

        // We might be able to do smart things. But honestly, the lists are small. We can just retry
        // with an ignored index if the initial check fails :shrug:.

        (0..self.levels.len()).any(|idx| self._is_safe_internal(Some(idx)))
    }
}

impl FromStr for Report {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        let mut levels = vec![];
        while !parser.is_exhausted() {
            levels.push(parser.usize()?);
        }

        Ok(Self { levels })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day02::{parse_input, Report};

    const TEST_INPUT: &str = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9\n\
    ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());

        let reports = result.unwrap();
        assert_eq!(reports.len(), 6);
        assert_eq!(reports[0], Report { levels: vec![7, 6, 4, 2, 1] });
        assert_eq!(reports[1], Report { levels: vec![1, 2, 7, 8, 9] });
    }

    #[test]
    fn test_report_is_safe() {
        assert_eq!(Report { levels: vec![7, 6, 4, 2, 1] }.is_safe(), true);
        assert_eq!(Report { levels: vec![1, 2, 7, 8, 9] }.is_safe(), false);
        assert_eq!(Report { levels: vec![9, 7, 6, 2, 1] }.is_safe(), false);
        assert_eq!(Report { levels: vec![1, 3, 2, 4, 5] }.is_safe(), false);
        assert_eq!(Report { levels: vec![8, 6, 4, 4, 1] }.is_safe(), false);
        assert_eq!(Report { levels: vec![1, 3, 6, 7, 9] }.is_safe(), true);
    }


    #[test]
    fn test_report_is_safe_dampening() {
        assert_eq!(Report { levels: vec![7, 6, 4, 2, 1] }.is_safe_dampening(), true);
        assert_eq!(Report { levels: vec![1, 2, 7, 8, 9] }.is_safe_dampening(), false);
        assert_eq!(Report { levels: vec![9, 7, 6, 2, 1] }.is_safe_dampening(), false);
        assert_eq!(Report { levels: vec![1, 3, 2, 4, 5] }.is_safe_dampening(), true);
        assert_eq!(Report { levels: vec![8, 6, 4, 4, 1] }.is_safe_dampening(), true);
        assert_eq!(Report { levels: vec![1, 3, 6, 7, 9] }.is_safe_dampening(), true);
    }
}