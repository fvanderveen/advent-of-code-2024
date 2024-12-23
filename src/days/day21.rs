use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter, Write};
use std::iter::Iterator;
use crate::days::Day;
use crate::util::geometry::{Directions, Point};
use crate::util::number::parse_usize;

pub const DAY21: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let codes = input.lines().collect::<Vec<_>>();

    let total_complexity: usize = codes.iter().map(|c| get_code_cost(c, 2) * get_code_value(c)).sum();
    println!("Total complexity (first member) = {}", total_complexity);
}

fn puzzle2(input: &String) {
    let codes = input.lines().collect::<Vec<_>>();

    let total_complexity: usize = codes.iter().map(|c| get_code_cost(c, 25) * get_code_value(c)).sum();
    println!("Total complexity (second member) = {}", total_complexity);
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Move {
    Up, Down, Left, Right, Activate
}

impl Move {
    fn translate(&self, point: &Point) -> Point {
        match self {
            Move::Up => point.translate_in_direction(&Directions::Top, 1),
            Move::Down => point.translate_in_direction(&Directions::Bottom, 1),
            Move::Left => point.translate_in_direction(&Directions::Left, 1),
            Move::Right => point.translate_in_direction(&Directions::Right, 1),
            Move::Activate => *point
        }
    }
}

// New approach, shortest path by calculating the required moves (using some state) on the last keypad
// Nothing else seems to comply for the actual input :(
// Basically move from A -> number, and for every move compute how many inputs that requires (with the two levels of indirection)

fn get_code_value(code: &str) -> usize {
    let numeric_part = code.chars().take_while(|c| c.is_numeric()).collect::<String>();
    parse_usize(&numeric_part).unwrap()
}

type CostMap = HashMap<(Move, Move), usize>;

struct Moves {
    point: Point,
    cost: usize,
    last: Move,
    ups: usize,
    rights: usize,
    lefts: usize,
    downs: usize
}

impl Moves {
    fn complete(&self, cost_map: &CostMap) -> Option<usize> {
        if self.lefts == 0 && self.rights == 0 && self.ups == 0 && self.downs == 0 {
            Some(self.cost + cost_map[&(self.last, Move::Activate)])
        } else {
            None
        }
    }

    fn get_next_options(self, cost_map: &CostMap) -> Vec<Moves> {
        let mut result = vec![];
        if self.lefts > 0 { result.push(Moves { cost: self.cost + cost_map[&(self.last, Move::Left)], point: Move::Left.translate(&self.point), last: Move::Left, lefts: self.lefts - 1, ..self}) }
        if self.rights > 0 { result.push(Moves { cost: self.cost + cost_map[&(self.last, Move::Right)], point: Move::Right.translate(&self.point), last: Move::Right, rights: self.rights - 1, ..self}) }
        if self.ups > 0 { result.push(Moves { cost: self.cost + cost_map[&(self.last, Move::Up)], point: Move::Up.translate(&self.point), last: Move::Up, ups: self.ups - 1, ..self}) }
        if self.downs > 0 { result.push(Moves { cost: self.cost + cost_map[&(self.last, Move::Down)], point: Move::Down.translate(&self.point), last: Move::Down, downs: self.downs - 1, ..self}) }
        result
    }

    fn get_best_cost(from_point: &Point, to_point: &Point, allowed_points: &Vec<Point>, cost_map: &CostMap) -> Option<usize> {
        let up = if to_point.y < from_point.y { from_point.y - to_point.y } else { 0 };
        let right = if to_point.x > from_point.x { to_point.x - from_point.x } else { 0 };
        let left = if to_point.x < from_point.x { from_point.x - to_point.x } else { 0 };
        let down = if to_point.y > from_point.y { to_point.y - from_point.y } else { 0 };

        let mut queue = VecDeque::new();
        queue.push_back(Moves { cost: 0, point: *from_point, last: Move::Activate, ups: up as usize, rights: right as usize, lefts: left as usize, downs: down as usize });

        let mut best_option: Option<usize> = None;

        while let Some(moves) = queue.pop_front() {
            // If the move moved onto empty space, skip it
            if !allowed_points.contains(&moves.point) { continue }

            if let Some(cost) = moves.complete(&cost_map) {
                match best_option {
                    Some(v) if v < cost => { continue }, // Current found best is better
                    _ => best_option = Some(cost),
                }
            }

            // Just. Try. Everything.
            for mov in moves.get_next_options(&cost_map) {
                queue.push_back(mov);
            }
        }

        best_option
    }
}

fn get_code_cost(code: &str, number_of_controllers: usize) -> usize {
    let number_locations = HashMap::from([
        ('7', Point { x: 0, y: 0 }), ('8', Point { x: 1, y: 0 }), ('9', Point { x: 2, y: 0 }),
        ('4', Point { x: 0, y: 1 }), ('5', Point { x: 1, y: 1 }), ('6', Point { x: 2, y: 1 }),
        ('1', Point { x: 0, y: 2 }), ('2', Point { x: 1, y: 2 }), ('3', Point { x: 2, y: 2 }),
        ('0', Point { x: 1, y: 3 }), ('A', Point { x: 2, y: 3 })
    ]);
    let allowed_points = number_locations.values().cloned().collect::<Vec<_>>();

    let cost_map = build_move_cost(number_of_controllers);
    let mut cost = 0;

    for i in 0..code.len() {
        let from = if i == 0 { 'A' } else { code.chars().nth(i - 1).unwrap() };
        let to = code.chars().nth(i).unwrap();

        // For numbers we have the gap on the bottom-left, to avoid that; we move up before left
        // and we move right before down. (So up->right->left->down)
        let from_point = number_locations.get(&from).unwrap();
        let to_point = number_locations.get(&to).unwrap();

        cost += Moves::get_best_cost(from_point, to_point, &allowed_points, &cost_map).unwrap(); // Should be found
    }

    cost
}

// Part two needs a _lot_ of controllers in between. Let's see if we can rewrite to just count moves
// Upside, any controller move can be pre-calculated.
// The main controller can press buttons directly (1 move/move)
// The controller controlled by the main controller can be computed how much moves cost
// i.e: after activate; pressing up would be [left, activate], left would be [down, left, left, activate]
//  but from up, left would be [down, left, activate], while another up would just be [activate]
// The controller below that, would basically repeat the thing.
// i.e. activate -> up would be [left, activate], which translates to [activate -> press left, left -> press activate]
// we can compute this easily for the whole depth, yielding a map of what a move on the numpad would cost.
fn build_move_cost(number_of_controllers: usize) -> HashMap<(Move, Move), usize> {
    // for n = 1 (only main controller) the map would be (any, any) => 1
    // for n = 2, get the n = 1 controller, and create the new map using the shortest path between moves using the map
    // for n = 3, get the n = 2 controller, ...
    // ...
    let cost_map = if number_of_controllers == 1 { HashMap::new() } else { build_move_cost(number_of_controllers - 1) };

    let control_locations = HashMap::from([
        /* x:0, y:0  */ (Move::Up, Point { x: 1, y: 0 }), (Move::Activate, Point { x: 2, y: 0 }),
        (Move::Left, Point { x: 0, y: 1 }), (Move::Down, Point { x: 1, y: 1 }), (Move::Right, Point { x: 2, y: 1 })
    ]);
    let allowed_points = control_locations.values().cloned().collect::<Vec<_>>();

    let mut new_map = HashMap::new();

    let all_moves = vec![Move::Activate, Move::Up, Move::Down, Move::Left, Move::Right];
    for from in &all_moves {
        for to in &all_moves {
            // Iterate in pairs, getting the cost to move between them using the map.
            let from_point = control_locations.get(&from).unwrap();
            let to_point = control_locations.get(&to).unwrap();

            // If the first remove controller, the cost is 1 per move (+1 to activate), otherwise it should be found using the previous map.
            let cost = if number_of_controllers == 1 { from_point.manhattan_distance(to_point) as usize + 1 } else { Moves::get_best_cost(from_point, to_point, &allowed_points, &cost_map).unwrap() };
            new_map.insert((*from, *to), cost);
        }
    }

    new_map
}

#[cfg(test)]
mod tests {
    use crate::days::day21::{build_move_cost, get_code_cost, Move};

    #[test]
    fn test_build_move_cost_map() {
        let c1_map = build_move_cost(1);
        assert_eq!(c1_map.get(&(Move::Activate, Move::Left)), Some(&4)); // down, left, left, activate

        // c2 should end up for 379A to be v<<A>>^A|vA^A|<vA<AA>>^AA|vA<^A>AA|vA^A|<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        let c2_map = build_move_cost(2);
        // To get from A -> 3,
        // A->Up (v<<A>>^A) (left, activate)
        assert_eq!(c2_map[&(Move::Activate, Move::Up)], 8);
        // Up->A (vA^A) (right, activate)
        assert_eq!(c2_map[&(Move::Up, Move::Activate)], 4);
        // 3 -> 7 <vA(down)<AA(left,left)>>^AA(activate,activate) (2x left) vA(right)<^A(up)>AA(activate,activate) (2x up) vA^A(right,activate) (activate)
        // Best options...
        assert_eq!(c2_map[&(Move::Activate, Move::Left)], 10);
        assert_eq!(c2_map[&(Move::Left, Move::Left)], 1);
        assert_eq!(c2_map[&(Move::Left, Move::Up)], 7);
        assert_eq!(c2_map[&(Move::Up, Move::Up)], 1);
        assert_eq!(c2_map[&(Move::Up, Move::Activate)], 4);
        // My options...
        // A -> Up v<<A>>^A(left, activate)
        assert_eq!(c2_map[&(Move::Activate, Move::Up)], 8);
        // Up -> Up A
        assert_eq!(c2_map[&(Move::Up, Move::Up)], 1);
        // Up -> Left v<A<A>>^A (down, left, activate)
        assert_eq!(c2_map[&(Move::Up, Move::Left)], 9);
        // Left -> Left A
        assert_eq!(c2_map[&(Move::Left, Move::Left)], 1);
        // Left -> A vAA(right,right)<^A(up)>A(activate)
        assert_eq!(c2_map[&(Move::Left, Move::Activate)], 8);
        // 7 -> 9
        // A -> Right
        // Right -> Right
        // Right -> A
        // 9 -> A
        // A -> Down
        // Down -> Down
        // Down -> Down
        // Donw -> A
    }

    #[test]
    fn test_get_code_cost() {
        assert_eq!(get_code_cost("029A", 2), 68);
        assert_eq!(get_code_cost("980A", 2), 60);
        assert_eq!(get_code_cost("179A", 2), 68);
        assert_eq!(get_code_cost("456A", 2), 64);
        assert_eq!(get_code_cost("379A", 2), 64);
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Up => f.write_char('^'),
            Move::Down => f.write_char('v'),
            Move::Left => f.write_char('<'),
            Move::Right => f.write_char('>'),
            Move::Activate => f.write_char('A'),
        }
    }
}