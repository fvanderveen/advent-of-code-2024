use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY4: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let grid = parse_input(input).unwrap();
    let xmas_count = xmas_finder(&grid);

    println!("There are {} occurrences of the word 'XMAS'", xmas_count);
}

fn puzzle2(input: &String) {
    let grid = parse_input(input).unwrap();
    let x_mas_count = x_mas_finder(&grid);

    println!("There are {} occurrences of X-MAS", x_mas_count);
}

fn parse_input(input: &str) -> Result<Grid<char>, String> {
    input.parse()
}

fn xmas_finder(puzzle: &Grid<char>) -> usize {
    // Find all occurrences of 'XMAS', letters can be used more than once.
    // Battle plan:
    // - Take all 'X' as starting points
    // - Do a search in all directions trying to match 'M', 'A', and 'S'

    let search_directions = vec![
        Directions::Top, Directions::Right, Directions::Bottom, Directions::Left,
        Directions::TopRight, Directions::BottomRight, Directions::BottomLeft, Directions::TopLeft
    ];

    let mut found = 0;

    let starting_points: Vec<(Point, char)> = puzzle.entries().iter().filter(|(_, c)| 'X'.eq(c)).cloned().collect();

    for (p, _) in starting_points {
        for dir in &search_directions {
            let chars: Vec<char> = puzzle.get_in_direction(&p, *dir).iter().take(3).map(|c| *c).collect();
            match chars[..] {
                ['M', 'A', 'S'] => found = found + 1,
                _ => {}
            }
        }
    }

    found
}

fn x_mas_finder(puzzle: &Grid<char>) -> usize {
    // Instead of finding 'XMAS', we need to find 'MAS' in a cross
    // M.S
    // .A.
    // M.S
    // Battle plan: get all 'A', and check if [TL,BR] ~= [M,S] and [TR,BL] ~= [M,S]

    let starting_points: Vec<(Point, char)> = puzzle.entries().into_iter().filter(|(_, c)| 'A'.eq(c)).collect();

    let mut found = 0;

    for (p, _) in starting_points {
        let tlbr: Result<[char; 2], _> = puzzle.get_adjacent(&p, Directions::TLBR).try_into();
        let trbl: Result<[char; 2], _> = puzzle.get_adjacent(&p, Directions::TRBL).try_into();

        match (tlbr, trbl) {
            (Ok(['M', 'S'] | ['S', 'M']), Ok(['M', 'S'] | ['S', 'M'])) => found = found + 1,
            _ => {}
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use crate::days::day04::{parse_input, x_mas_finder, xmas_finder};

    const TEST_INPUT: &str = "\
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX\n\
    ";

    #[test]
    fn test_xmas_finder() {
        let grid = parse_input(TEST_INPUT).unwrap();

        assert_eq!(xmas_finder(&grid), 18);
    }

    #[test]
    fn test_x_mas_finder() {
        let grid = parse_input(TEST_INPUT).unwrap();

        assert_eq!(x_mas_finder(&grid), 9);
    }
}