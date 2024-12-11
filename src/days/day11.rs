use std::collections::HashMap;
use crate::days::Day;
use crate::util::number::parse_usize;

pub const DAY11: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let stones = input.split(" ").map(parse_usize).collect::<Result<Vec<_>, _>>().unwrap();

    println!("After blinking 25 times, we see {} stones", run_stone_simulation(stones, 25).unwrap());
}

fn puzzle2(input: &String) {
    let stones = input.split(" ").map(parse_usize).collect::<Result<Vec<_>, _>>().unwrap();

    println!("After blinking 75 times, we see {} stones", run_stone_simulation(stones, 75).unwrap());
}

fn run_stone_simulation(input: Vec<usize>, blinks: usize) -> Result<usize, String> {
    let mut cache = HashMap::new();
    input.into_iter().map(|seed| get_number_of_stones_after(seed, blinks, &mut cache)).sum()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct CacheKey {
    seed: usize,
    blinks: usize,
}

fn get_number_of_stones_after(seed: usize, blinks: usize, cache: &mut HashMap<CacheKey, usize>) -> Result<usize, String> {
    // - If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    // - If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
    //   The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone.
    //   (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    // - If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
    // (Wanna bet we need a cache for the second part?)

    if blinks == 0 { return Ok(1); }

    let key = CacheKey { seed, blinks };
    if let Some(cached) = cache.get(&key) { return Ok(*cached); }

    let as_str = seed.to_string();

    let answer = match as_str.as_str() {
        "0" => get_number_of_stones_after(1, blinks - 1, cache)?,
        _ if (as_str.len() % 2) == 0 => {
            let first_half = parse_usize(&as_str[..(as_str.len() / 2)])?;
            let second_half = parse_usize(&as_str[(as_str.len() / 2)..])?;
            get_number_of_stones_after(first_half, blinks - 1, cache)? + get_number_of_stones_after(second_half, blinks - 1, cache)?
        }
        _ => get_number_of_stones_after(seed * 2024, blinks - 1, cache)?
    };

    cache.insert(key, answer);
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::days::day11::{get_number_of_stones_after, run_stone_simulation};

    #[test]
    fn test_get_number_of_stones_after() {
        assert_eq!(get_number_of_stones_after(0, 1, &mut HashMap::new()), Ok(1));
        assert_eq!(get_number_of_stones_after(1, 1, &mut HashMap::new()), Ok(1));
        assert_eq!(get_number_of_stones_after(10, 1, &mut HashMap::new()), Ok(2));
        assert_eq!(get_number_of_stones_after(10, 2, &mut HashMap::new()), Ok(2));
        assert_eq!(get_number_of_stones_after(10, 3, &mut HashMap::new()), Ok(3));
        assert_eq!(get_number_of_stones_after(2024, 1, &mut HashMap::new()), Ok(2));

        assert_eq!(get_number_of_stones_after(125, 1, &mut HashMap::new()), Ok(1));
        assert_eq!(get_number_of_stones_after(125, 2, &mut HashMap::new()), Ok(2));
        assert_eq!(get_number_of_stones_after(125, 3, &mut HashMap::new()), Ok(2));
        assert_eq!(get_number_of_stones_after(125, 4, &mut HashMap::new()), Ok(3));
        assert_eq!(get_number_of_stones_after(125, 5, &mut HashMap::new()), Ok(5));
        assert_eq!(get_number_of_stones_after(125, 6, &mut HashMap::new()), Ok(7));

        let mut cache = HashMap::new();
        let first = get_number_of_stones_after(125, 25, &mut cache).unwrap();
        let second = get_number_of_stones_after(17, 25, &mut cache).unwrap();
        assert_eq!(first + second, 55312);
    }

    #[test]
    fn test_run_stone_simulation() {
        assert_eq!(run_stone_simulation(vec![125, 17], 25), Ok(55312));
    }
}