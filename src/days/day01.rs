use std::collections::HashMap;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY1: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let (left, right) = parse_input(input).unwrap();

    println!("Diff between lists: {}", distance_between_lists(&left, &right).unwrap());
}

fn puzzle2(input: &String) {
    let (left, right) = parse_input(input).unwrap();

    println!("Similarity of lists: {}", calculate_similarity(&left, &right));
}

fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<usize>), String> {
    let mut left = vec![];
    let mut right = vec![];

    let mut parser = Parser::new(input);
    while !parser.is_exhausted() {
        left.push(parser.usize()?);
        right.push(parser.usize()?);
    }

    Ok((left, right))
}

fn distance_between_lists(left: &Vec<usize>, right: &Vec<usize>) -> Result<usize, String> {
    if left.len() != right.len() {
        return Err("Given lists are not of the same length".to_string());
    }

    // To get the distance, we take the lowest number on both sides and compare the difference.
    // The total distance is the sum of those numbers.
    let mut sorted_left = left.clone();
    sorted_left.sort();
    let mut sorted_right = right.clone();
    sorted_right.sort();

    Ok((0..left.len()).map(|idx| sorted_left[idx].abs_diff(sorted_right[idx])).sum())
}

fn calculate_similarity(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    // Similarity is computed by taking each number in the left list, and multiplying its value by the number of occurrences in the right list.
    // To speed up this process, we create a lookup for right value => number of occurrences.

    let mut lookup: HashMap<usize, usize> = HashMap::new();

    for value in right {
        lookup.insert(*value, lookup.get(value).unwrap_or(&0) + 1);
    }

    left.iter().map(|v| v * lookup.get(v).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use crate::days::day01::{calculate_similarity, distance_between_lists, parse_input};

    const TEST_INPUT: &str = "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3\n\
    ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);

        assert!(result.is_ok());
        let (left, right) = result.unwrap();

        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_distance_between_lists() {
        let (left, right) = parse_input(TEST_INPUT).unwrap();

        assert_eq!(distance_between_lists(&left, &right), Ok(11));
    }

    #[test]
    fn test_calculate_similarity() {
        let (left, right) = parse_input(TEST_INPUT).unwrap();

        assert_eq!(calculate_similarity(&left, &right), 31);
    }
}