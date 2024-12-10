use std::collections::{HashSet, VecDeque};
use crate::days::Day;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY10: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let map: TrailMap = input.parse().unwrap();

    println!("Total trailhead scores: {}", map.get_total_score());
}

fn puzzle2(input: &String) {
    let map: TrailMap = input.parse().unwrap();

    println!("Total trailhead rating: {}", map.get_total_rating());
}

type TrailMap = Grid<usize>;

impl TrailMap {
    fn get_trailhead_scores(&self) -> Vec<(Point, (usize, usize))> {
        self.points().into_iter().filter_map(|p| {
            match self.get(&p) {
                Some(0) => Some((p, self.get_trailhead_score(p))),
                _ => None
            }
        }).collect()
    }

    fn get_trailhead_score(&self, point: Point) -> (usize, usize) {
        // Get the number of (different) '9' tiles we can reach from the given point.
        // We can move up/down/left/right and only to a tile 1 higher from the current.

        let mut peaks = HashSet::new();
        let mut paths = 0;
        let mut queue = VecDeque::new();
        queue.push_back(point);

        while let Some(point) = queue.pop_front() {
            // Get points around this point with a value of 1 higher.
            if let Some(cur_value) = self.get(&point) {
                self.get_adjacent_entries(&point, Directions::NonDiagonal).into_iter().filter(|(_, v)| (cur_value + 1).eq(v))
                    .for_each(|(p, v)| {
                        if v == 9 {
                            peaks.insert(p);
                            paths += 1;
                        } else {
                            queue.push_back(p);
                        }
                    });
            }
        }

        (peaks.len(), paths)
    }

    fn get_total_score(&self) -> usize {
        self.get_trailhead_scores().into_iter().map(|(_, (v, _))| v).sum()
    }

    fn get_total_rating(&self) -> usize {
        self.get_trailhead_scores().into_iter().map(|(_, (_, v))| v).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day10::TrailMap;

    const TEST_INPUT: &str = "\
        89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732\n\
    ";

    #[test]
    fn test_get_trailhead_scores() {
        let map: TrailMap = TEST_INPUT.parse().unwrap();

        let trailheads = map.get_trailhead_scores();
        assert_eq!(trailheads, vec![
            ((2,0).into(), (5, 20)),
            ((4,0).into(), (6, 24)),
            ((4,2).into(), (5, 10)),
            ((6,4).into(), (3, 4)),
            ((2,5).into(), (1, 1)),
            ((5,5).into(), (3, 4)),
            ((0,6).into(), (5, 5)),
            ((6,6).into(), (3, 8)),
            ((1,7).into(), (5, 5)),
        ]);
    }

    #[test]
    fn test_get_total_score() {
        let map: TrailMap = TEST_INPUT.parse().unwrap();

        assert_eq!(map.get_total_score(), 36);
    }

    #[test]
    fn test_get_total_rating() {
        let map: TrailMap = TEST_INPUT.parse().unwrap();

        assert_eq!(map.get_total_rating(), 81);
    }
}