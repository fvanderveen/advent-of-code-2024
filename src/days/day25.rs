use std::str::FromStr;
use crate::days::Day;
use crate::util::geometry::Grid;

pub const DAY25: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let schematics = parse_input(input).unwrap();

    println!("There are {} unique lock/key combos that fit.", get_possible_lock_key_combos(&schematics));
}

fn puzzle2(_input: &String) {
    println!("Freebie for Christmas~");
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Schematic {
    Lock([usize; 5]),
    Key([usize; 5])
}

fn parse_input(input: &str) -> Result<Vec<Schematic>, String> {
    input.split("\n\n").map(|i| i.parse()).collect::<Result<Vec<_>, _>>()
}

fn get_possible_lock_key_combos(schematics: &Vec<Schematic>) -> usize {
    let locks = schematics.iter().filter_map(|s| match s {
        Schematic::Lock(l) => Some(l),
        _ => None
    }).copied().collect::<Vec<_>>();
    let keys = schematics.iter().filter_map(|s| match s {
        Schematic::Key(l) => Some(l),
        _ => None
    }).copied().collect::<Vec<_>>();

    let mut result = 0;

    for lock in &locks {
        'key_loop: for key in &keys {
            for i in 0..5 {
                if lock[i] + key[i] > 5 { continue 'key_loop; } // This key aint gonna fit
            }
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::days::day25::{get_possible_lock_key_combos, parse_input, Schematic};

    #[test]
    fn test_parse() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());

        let schematics = result.unwrap();
        assert_eq!(schematics, vec![
            Schematic::Lock([0, 5, 3, 4, 3]),
            Schematic::Lock([1, 2, 0, 5, 3]),
            Schematic::Key([5, 0, 2, 1, 3]),
            Schematic::Key([4, 3, 4, 0, 2]),
            Schematic::Key([3, 0, 2, 0, 1]),
        ])
    }

    #[test]
    fn test_get_possible_lock_key_combos() {
        let schematics = parse_input(TEST_INPUT).unwrap();
        assert_eq!(get_possible_lock_key_combos(&schematics), 3);
    }

    const TEST_INPUT: &str = "\
        #####\n\
        .####\n\
        .####\n\
        .####\n\
        .#.#.\n\
        .#...\n\
        .....\n\
        \n\
        #####\n\
        ##.##\n\
        .#.##\n\
        ...##\n\
        ...#.\n\
        ...#.\n\
        .....\n\
        \n\
        .....\n\
        #....\n\
        #....\n\
        #...#\n\
        #.#.#\n\
        #.###\n\
        #####\n\
        \n\
        .....\n\
        .....\n\
        #.#..\n\
        ###..\n\
        ###.#\n\
        ###.#\n\
        #####\n\
        \n\
        .....\n\
        .....\n\
        .....\n\
        #....\n\
        #.#..\n\
        #.#.#\n\
        #####\n\
    ";
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<char> = s.parse()?;

        if grid.bounds.width != 5 { return Err(format!("Invalid input width {}", grid.bounds.width)); }
        if grid.bounds.height != 7 { return Err(format!("Invalid input height {}", grid.bounds.height)); }

        let is_lock = grid.bounds.x().all(|x| grid.get(&(x, 0).into()) == Some('#'));
        let is_key = grid.bounds.x().all(|x| grid.get(&(x, 6).into()) == Some('#'));
        if is_lock == is_key { return Err(format!("Could not determine if input is a lock or key?!\n{}", s)); }

        let mut heights: [usize; 5] = Default::default();
        for x in grid.bounds.x() {
            let ys = grid.bounds.y();
            if is_key {
                heights[x as usize] = ys.rev().take_while(|y| grid.get(&(x, *y).into()) == Some('#')).count() - 1; // skip the border in height calcs
            } else {
                heights[x as usize] = ys.take_while(|y| grid.get(&(x, *y).into()) == Some('#')).count() - 1;
            }

        }

        if is_lock { Ok(Self::Lock(heights)) } else { Ok(Self::Key(heights)) }
    }
}