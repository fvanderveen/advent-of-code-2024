use std::collections::HashMap;
use crate::days::Day;

pub const DAY19: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let towels: Towels = input.as_str().into();

    let possible_designs = towels.get_possible_designs();
    println!("We can creates {} of the designs.", possible_designs.len());
}

fn puzzle2(input: &String) {
    let towels: Towels = input.as_str().into();

    println!("The possible designs can be stacked in {} different ways.", towels.get_possible_design_arrangements());
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Towels<'a> {
    available_patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl <'a> Towels<'_> {
    fn stack_design_options<'b>(&self, design: &'b str, design_cache: &mut HashMap<&'b str, usize>) -> usize {
        if let Some(option_count) = design_cache.get(design) {
            return *option_count;
        }

        // find options in our patterns that design starts with
        let options: Vec<_> = self.available_patterns.iter().filter(|p| design.starts_with(*p)).collect();

        let mut result = 0;

        // for each option, check if we can complete it by recursively checking
        for option in options {
            let left_over = &design[option.len()..];

            match left_over {
                "" => result += 1,
                _ => result += self.stack_design_options(left_over, design_cache),
            }
        }

        design_cache.insert(design, result);

        result
    }

    fn get_possible_designs(&'a self) -> Vec<&'a str> {
        let mut design_cache = HashMap::new();
        self.designs.clone().into_iter().filter(|d| self.stack_design_options(d, &mut design_cache) != 0).collect()
    }

    fn get_possible_design_arrangements(&self) -> usize {
        let mut design_cache = HashMap::new();
        self.designs.iter().map(|d| self.stack_design_options(d, &mut design_cache)).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day19::Towels;

    const TEST_INPUT: &str = "\
        r, wr, b, g, bwu, rb, gb, br\n\
        \n\
        brwrr\n\
        bggr\n\
        gbbr\n\
        rrbgbr\n\
        ubwu\n\
        bwurrg\n\
        brgr\n\
        bbrgwb\n\
    ";

    #[test]
    fn test_get_possible_designs() {
        let towels: Towels = TEST_INPUT.into();

        assert_eq!(towels.get_possible_designs(), vec![
            "brwrr",
            "bggr",
            "gbbr",
            "rrbgbr",
            "bwurrg",
            "brgr"
        ]);
    }

    #[test]
    fn test_get_possible_design_arrangements() {
        let towels: Towels = TEST_INPUT.into();

        assert_eq!(towels.get_possible_design_arrangements(), 16);
    }
}

impl <'a> From<&'a str> for Towels<'a> {
    fn from(s: &'a str) -> Self {
        let lines: Vec<_> = s.lines().collect();

        // First line is a comma-separated list of available towel patterns
        let available_patterns = lines[0].split(",").map(|p| p.trim()).collect();

        // Third line and on are designs:
        let designs = lines[2..].into_iter().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();

        Self { available_patterns, designs }
    }
}