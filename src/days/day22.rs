use std::collections::{HashMap, HashSet};
use std::ops::BitXor;
use crate::days::Day;
use crate::util::number::parse_usize;

pub const DAY22: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let result: usize = input.lines().filter_map(|l| parse_usize(l).ok()).map(|v| get_nth_number(v, 2000)).sum();
    println!("The sum of the 2000th numbers is {}", result);
}

fn puzzle2(input: &String) {
    let seeds = input.lines().filter_map(|l| parse_usize(l).ok()).collect::<Vec<_>>();
    let best_result = get_best_income(seeds);
    println!("The most bananas to get: {}", best_result);
}

fn get_next_secret_number(number: usize) -> usize {
    let first = number.bitxor(number * 64) % 16777216;
    let second = first.bitxor(first / 32) % 16777216;
    second.bitxor(second * 2048) % 16777216
}

fn get_nth_number(seed: usize, n: usize) -> usize {
    let mut current = seed;
    for _ in 0..n {
        current = get_next_secret_number(current);
    }
    current
}

fn get_best_income(seeds: Vec<usize>) -> usize {
    // For every seed, we get 2000 numbers. The price is the last digit (%10), and we need the
    // sequence of 4 changes. Using a rolling window we can store each in a map with the value it
    // would give, and then find the highest value.
    let mut winnings: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();

    for seed in seeds {
        let mut secret = seed;
        let mut last_differences = vec![];

        // Local map so we can ignore sequences that happen more than once (only the first counts per seed)
        let mut seen_sequences = HashSet::new();

        for _ in 0..2000 {
            let old_price = secret % 10;
            secret = get_next_secret_number(secret);
            let next_price = secret % 10;
            let difference = next_price as isize - old_price as isize;

            if last_differences.len() < 3 {
                last_differences.push(difference)
            } else {
                let [a, b, c] = last_differences[..] else { unreachable!("sliding window had more than 4 items?!") };
                let key = (a, b, c, difference);

                if seen_sequences.insert(key) {
                    winnings.insert(key, winnings.get(&key).unwrap_or(&0) + next_price);
                }

                last_differences = vec![b, c, difference];
            }
        }
    }

    *winnings.values().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use crate::days::day22::{get_best_income, get_next_secret_number, get_nth_number};

    #[test]
    fn test_get_nth_number() {
        assert_eq!(get_nth_number(1, 2000), 8685429);
        assert_eq!(get_nth_number(10, 2000), 4700978);
        assert_eq!(get_nth_number(100, 2000), 15273692);
        assert_eq!(get_nth_number(2024, 2000), 8667524);
    }

    #[test]
    fn test_get_best_income() {
        assert_eq!(get_best_income(vec![1, 2, 3, 2024]), 23)
    }

    #[test]
    fn test_get_next_secret_number() {
        assert_eq!(get_next_secret_number(123), 15887950);
        assert_eq!(get_next_secret_number(15887950), 16495136);
        assert_eq!(get_next_secret_number(16495136), 527345);
        assert_eq!(get_next_secret_number(527345), 704524);
        assert_eq!(get_next_secret_number(704524), 1553684);
        assert_eq!(get_next_secret_number(1553684), 12683156);
        assert_eq!(get_next_secret_number(12683156), 11100544);
        assert_eq!(get_next_secret_number(11100544), 12249484);
        assert_eq!(get_next_secret_number(12249484), 7753432);
        assert_eq!(get_next_secret_number(7753432), 5908254);
    }
}