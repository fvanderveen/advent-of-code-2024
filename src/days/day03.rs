use regex::Regex;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY3: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let result = puzzle1_parse(input).unwrap();
    println!("Sum of all valid mul instructions: {}", result);
}

fn puzzle2(input: &String) {
    let result = puzzle2_parse(input).unwrap();
    println!("Sum of all valid mul instructions, accounting for conditionals: {}", result);
}

fn puzzle1_parse(input: &str) -> Result<usize, String> {
    // Scan input for valid `mul(#[##],#[##])` instructions, execute them and return the sum.
    let regex = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").map_err(|e| e.to_string())?;

    Ok(regex.find_iter(input).map(|mat| parse_mul_instr(mat.as_str())).collect::<Result<Vec<usize>, String>>()?.iter().sum())
}

fn parse_mul_instr(input: &str) -> Result<usize, String> {
    let mut parser = Parser::new(input);
    parser.literal("mul(")?;
    let left = parser.usize()?;
    parser.literal(",")?;
    let right = parser.usize()?;
    parser.literal(")")?;
    parser.ensure_exhausted()?;

    Ok(left * right)
}

fn puzzle2_parse(input: &str) -> Result<usize, String> {
    let mut current_index = 0;
    let mul_regex = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").map_err(|e| e.to_string())?;
    let do_regex = Regex::new("do\\(\\)").map_err(|e| e.to_string())?;
    let dont_regex = Regex::new("don't\\(\\)").map_err(|e| e.to_string())?;

    let mut result = 0;
    let mut mul_enabled = true;
    // if mul_enabled, search next mul(...) and don't() indexes, take closest.
    // if not mul_enabled, search next do() for continuing

    loop {
        if !mul_enabled {
            match do_regex.find_at(input, current_index) {
                None => { return Ok(result); } // we're done, no more enabling in the string.
                Some(mat) => {
                    mul_enabled = true;
                    current_index = mat.end(); // skip the do() and continue the loop.
                    continue;
                }
            }
        }

        // Mul enabled
        let mul_index = mul_regex.find_at(input, current_index);
        let dont_index = dont_regex.find_at(input, current_index);

        match (mul_index, dont_index) {
            (None, _) => { return Ok(result); } // if we have no mul instructions left, we don't need to follow other dos and don'ts.
            (Some(mul), None) => {
                result += parse_mul_instr(mul.as_str())?;
                current_index = mul.end();
            }
            (Some(mul), Some(dont)) => {
                if mul.start() < dont.start() {
                    result += parse_mul_instr(mul.as_str())?;
                    current_index = mul.end();
                } else {
                    mul_enabled = false;
                    current_index = dont.end();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day03::{puzzle1_parse, puzzle2_parse};

    const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_puzzle1_parse() {
        assert_eq!(puzzle1_parse(TEST_INPUT), Ok(161));
    }

    #[test]
    fn test_puzzle2_parse() {
        // TEST_INPUT has no don'ts, so the result should be the same as before.
        assert_eq!(puzzle2_parse(TEST_INPUT), Ok(161));
        assert_eq!(puzzle2_parse(TEST_INPUT_2), Ok(48));
    }
}