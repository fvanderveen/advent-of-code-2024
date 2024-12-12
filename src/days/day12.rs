use std::collections::{HashSet, VecDeque};
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY12: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let garden: Garden = input.parse().unwrap();

    println!("The total cost of adding fences to this garden is {}", garden.get_total_price());
}

fn puzzle2(input: &String) {
    let garden: Garden = input.parse().unwrap();

    println!("The bulk cost of adding fences to this garden is {}", garden.get_bulk_price());
}

type Garden = Grid<char>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Region {
    name: char,
    area: usize,
    perimeter: usize,
    sides: usize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Fence {
    side: Directions,
    points: Vec<Point>,
}

impl Garden {
    fn get_regions(&self) -> Vec<Region> {
        // Count area and perimeter for every Region, duplicate names could occur
        let mut regions: Vec<Region> = vec![];
        let mut handled_cells: HashSet<Point> = HashSet::new();

        for (point, name) in self.entries() {
            if !handled_cells.insert(point) { continue; } // Already visited for a region

            fn handle_side(garden: &Garden, point: Point, side: Directions, name: char, fences: &mut Vec<Fence>) -> Option<Point> {
                let sibling = point.translate_in_direction(&side, 1);

                match garden.get(&sibling) {
                    Some(plot) if plot == name => {
                        // Same region, return as new point to handle
                        Some(sibling)
                    },
                    _ => {
                        // Other region (or boundary). Add fence (and possibly merge other fences)
                        let mut fence = Fence { side, points: vec![point] };

                        fences.retain(|f| !fence.merge(f));
                        fences.push(fence);

                        None
                    }
                }
            }

            // Got a new region.

            let mut queue: VecDeque<Point> = VecDeque::new();
            queue.push_back(point);

            let mut fences = vec![];
            let mut area = 0;

            while let Some(current) = queue.pop_front() {
                area += 1;

                for side in vec![Directions::Top, Directions::Right, Directions::Bottom, Directions::Left] {
                    if let Some(next_cell) = handle_side(self, current, side, name, &mut fences) {
                        if handled_cells.insert(next_cell) {
                            queue.push_back(next_cell);
                        }
                    }
                }
            }

            let perimeter = fences.iter().map(|f| f.len()).sum();
            let sides = fences.len();

            regions.push(Region { name, area, perimeter, sides });
        }

        regions.sort_by(|l, r| l.name.cmp(&r.name).then_with(|| l.area.cmp(&r.area)).then_with(|| l.perimeter.cmp(&r.perimeter)));

        regions
    }

    fn get_total_price(&self) -> usize {
        self.get_regions().iter().map(|r| r.area * r.perimeter).sum()
    }

    fn get_bulk_price(&self) -> usize {
        self.get_regions().iter().map(|r| r.area * r.sides).sum()
    }
}

impl Fence {
    fn get_neighbour_points(&self) -> Vec<Point> {
        match self.side {
            Directions::Top | Directions::Bottom => {
                // Get the left/right-most points and get those next to it.
                let left = self.points.iter().min_by_key(|p| p.x).map(|p| p.translate_in_direction(&Directions::Left, 1)).unwrap();
                let right = self.points.iter().max_by_key(|p| p.x).map(|p| p.translate_in_direction(&Directions::Right, 1)).unwrap();
                vec![left, right]
            },
            Directions::Left | Directions::Right => {
                // Get the top/bottom-most points and get those next to it.
                let top = self.points.iter().min_by_key(|p| p.y).map(|p| p.translate_in_direction(&Directions::Top, 1)).unwrap();
                let bottom = self.points.iter().max_by_key(|p| p.y).map(|p| p.translate_in_direction(&Directions::Bottom, 1)).unwrap();
                vec![top, bottom]
            },
            _ => unreachable!() // invalid direction.
        }
    }

    fn can_merge(&self, other: &Fence) -> bool {
        if self.side != other.side { return false; }

        self.get_neighbour_points().iter().any(|p| other.points.contains(p))
    }

    fn merge(&mut self, other: &Fence) -> bool {
        if self.can_merge(other) {
            self.points.push_all(&other.points);
            true
        } else { false }
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day12::{Garden, Region};

    const SMALL_TEST_INPUT: &str = "\
        AAAA\n\
        BBCD\n\
        BBCC\n\
        EEEC\n\
    ";

    const CONCAVE_TEST_INPUT: &str = "\
        OOOOO\n\
        OXOXO\n\
        OOOOO\n\
        OXOXO\n\
        OOOOO\n\
    ";

    const LARGE_TEST_INPUT: &str = "\
        RRRRIICCFF\n\
        RRRRIICCCF\n\
        VVRRRCCFFF\n\
        VVRCCCJFFF\n\
        VVVVCJJCFE\n\
        VVIVCCJJEE\n\
        VVIIICJJEE\n\
        MIIIIIJJEE\n\
        MIIISIJEEE\n\
        MMMISSJEEE\n\
    ";

    const E_SHAPE_TEST_INPUT: &str = "\
        EEEEE\n\
        EXXXX\n\
        EEEEE\n\
        EXXXX\n\
        EEEEE\n\
    ";

    const MOBIUS_TEST_INPUT: &str = "\
        AAAAAA\n\
        AAABBA\n\
        AAABBA\n\
        ABBAAA\n\
        ABBAAA\n\
        AAAAAA\n\
    ";

    #[test]
    fn test_get_regions() {
        let small_map: Garden = SMALL_TEST_INPUT.parse().unwrap();
        let small_regions = small_map.get_regions();

        assert_eq!(small_regions, vec![
            Region { name: 'A', area: 4, perimeter: 10, sides: 4 },
            Region { name: 'B', area: 4, perimeter: 8, sides: 4 },
            Region { name: 'C', area: 4, perimeter: 10, sides: 8 },
            Region { name: 'D', area: 1, perimeter: 4, sides: 4 },
            Region { name: 'E', area: 3, perimeter: 8, sides: 4 },
        ]);

        let concave_map: Garden = CONCAVE_TEST_INPUT.parse().unwrap();
        let concave_regions = concave_map.get_regions();

        assert_eq!(concave_regions, vec![
            Region { name: 'O', area: 21, perimeter: 36, sides: 20 },
            Region { name: 'X', area: 1, perimeter: 4, sides: 4 },
            Region { name: 'X', area: 1, perimeter: 4, sides: 4 },
            Region { name: 'X', area: 1, perimeter: 4, sides: 4 },
            Region { name: 'X', area: 1, perimeter: 4, sides: 4 },
        ]);

        let large_map: Garden = LARGE_TEST_INPUT.parse().unwrap();
        let large_regions = large_map.get_regions();

        assert_eq!(large_regions, vec![
            Region { name: 'C', area: 1, perimeter: 4, sides: 4 },
            Region { name: 'C', area: 14, perimeter: 28, sides: 22 },
            Region { name: 'E', area: 13, perimeter: 18, sides: 8 },
            Region { name: 'F', area: 10, perimeter: 18, sides: 12 },
            Region { name: 'I', area: 4, perimeter: 8, sides: 4 },
            Region { name: 'I', area: 14, perimeter: 22, sides: 16 },
            Region { name: 'J', area: 11, perimeter: 20, sides: 12 },
            Region { name: 'M', area: 5, perimeter: 12, sides: 6 },
            Region { name: 'R', area: 12, perimeter: 18, sides: 10 },
            Region { name: 'S', area: 3, perimeter: 8, sides: 6 },
            Region { name: 'V', area: 13, perimeter: 20, sides: 10 },
        ]);

        let e_map: Garden = E_SHAPE_TEST_INPUT.parse().unwrap();
        let e_regions = e_map.get_regions();

        assert_eq!(e_regions, vec![
            Region { name: 'E', area: 17, perimeter: 36, sides: 12 },
            Region { name: 'X', area: 4, perimeter: 10, sides: 4 },
            Region { name: 'X', area: 4, perimeter: 10, sides: 4 },
        ]);

        let mobius_map: Garden = MOBIUS_TEST_INPUT.parse().unwrap();
        let mobius_regions = mobius_map.get_regions();

        assert_eq!(mobius_regions, vec![
            Region { name: 'A', area: 28, perimeter: 40, sides: 12 },
            Region { name: 'B', area: 4, perimeter: 8, sides: 4 },
            Region { name: 'B', area: 4, perimeter: 8, sides: 4 },
        ]);
    }

    #[test]
    fn test_get_total_price() {
        let small_map: Garden = SMALL_TEST_INPUT.parse().unwrap();
        let concave_map: Garden = CONCAVE_TEST_INPUT.parse().unwrap();
        let large_map: Garden = LARGE_TEST_INPUT.parse().unwrap();

        assert_eq!(small_map.get_total_price(), 140);
        assert_eq!(concave_map.get_total_price(), 772);
        assert_eq!(large_map.get_total_price(), 1930);
    }

    #[test]
    fn test_get_bulk_price() {
        let small_map: Garden = SMALL_TEST_INPUT.parse().unwrap();
        let concave_map: Garden = CONCAVE_TEST_INPUT.parse().unwrap();
        let large_map: Garden = LARGE_TEST_INPUT.parse().unwrap();
        let e_map: Garden = E_SHAPE_TEST_INPUT.parse().unwrap();
        let mobius_map: Garden = MOBIUS_TEST_INPUT.parse().unwrap();

        assert_eq!(small_map.get_bulk_price(), 80);
        assert_eq!(concave_map.get_bulk_price(), 436);
        assert_eq!(large_map.get_bulk_price(), 1206);
        assert_eq!(e_map.get_bulk_price(), 236);
        assert_eq!(mobius_map.get_bulk_price(), 368);
    }
}