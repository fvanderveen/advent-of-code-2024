use std::collections::{HashMap};
use std::ops::BitXor;
use std::str::FromStr;
use crate::days::Day;
use crate::util::number::{parse_u8, parse_usize};

pub const DAY24: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let mut machine = Machine::parse(input).unwrap();
    machine.process_inputs();

    println!("Output: {}", machine.get_result());
}

fn puzzle2(input: &String) {
    let machine = Machine::parse(input).unwrap();
    let swapped = machine.find_swapped_outputs();

    println!("Swapped outputs: {}", swapped.join(","));
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Machine<'a> {
    wires: HashMap<&'a str, u8>,
    gates: Vec<Gate<'a>>
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Wire<'a> {
    name: &'a str,
    value: u8,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Operation {
    XOR, OR, AND
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Gate<'a> {
    inputs: [&'a str; 2],
    operation: Operation,
    output: &'a str
}

impl <'a> Machine<'a> {
    fn parse(input: &'a str) -> Result<Machine<'a>, String> {
        let [wires_input, gates_input] = input.split("\n\n").collect::<Vec<_>>()[..] else { return Err(format!("Invalid machine '{}'", input)) };

        let wire_values: Vec<Wire<'a>> = wires_input.lines().map(|l| Wire::parse(l)).collect::<Result<_, _>>()?;
        let mut wires = HashMap::new();
        for wire in wire_values {
            wires.insert(wire.name, wire.value);
        }

        let gates = gates_input.lines().map(|l| Gate::parse(l)).collect::<Result<_, _>>()?;

        Ok(Self { wires, gates })
    }

    fn process_inputs(&mut self) {
        // Run through the gates, process those where all inputs have a value.
        // Once all gates are processed, we're done (there are no loops)
        // If we can't process any gate in a loop, panic.
        let mut todo = self.gates.clone();

        while todo.len() > 0 {
            let mut handled = vec![];

            for i in 0..todo.len() {
                let gate = todo[i];
                let Some(left) = self.wires.get(gate.inputs[0]) else { continue };
                let Some(right) = self.wires.get(gate.inputs[1]) else { continue };

                self.wires.insert(gate.output, gate.operation.apply([left, right]));

                handled.push(i);
            }

            if handled.len() == 0 { panic!("No gates processed in loop!") }
            // Handle back-front (highest indexes first to remove the right items)
            while let Some(i) = handled.pop() {
                todo.remove(i);
            }
        }
    }

    fn get_result(&self) -> usize {
        self.get_wire_value("z")
    }

    fn get_wire_value(&self, wire: &str) -> usize {
        let mut current_bit = 0;
        let mut result = 0;

        while let Some(value) = self.wires.get(format!("{}{:02}", wire, current_bit).as_str()) {
            result += 2usize.pow(current_bit) * (*value as usize);
            current_bit += 1;
        }

        result
    }

    fn find_swapped_outputs(&self) -> Vec<&'a str> {
        // There are 8 outputs (4 pairs) swapped in the main output (test output has less)
        // Doing some investigations this should just be a 'simple' circuit adding values. (Ripple carry adder)
        // Every X and Y bit should be fed into a AND and XOR gate:
        // 101 + 011 = 1000
        // x00 (1), y00 (1) through XOR yield 0 (value) and through AND 1 (carry bit)
        // x01 (0), y01 (1) yields XOR 1 (value) and AND 0 (no carry).
        // x02 (1), y02 (0) yields XOR 1 (value) and AND 0 (no carry).
        // The next line of gates adds carries to the values of the next bits.
        // i.e. the 1 carry of xy00 is XORed with the 1 value of xy01 (and AND-ed to yield another carry)
        // As such, we know that:
        // a gate taking an x and y input needs to use XOR _or_ AND
        // a gate outputting to z needs to use XOR (except z45, which uses OR due to no carry-over)
        // any other gate can use OR or AND (to handle ripple carries)

        let x = self.get_wire_value("x");
        let y = self.get_wire_value("y");
        let z = x + y;

        // Get the z-out that should use the value of output (currently wired wrongly) by finding the z
        // that does use it through the carry (and subtract 1)
        fn first_z_that_should_use(machine: &Machine, output: &str) -> Option<String> {
            let gates = machine.gates.iter().filter(|g| g.inputs[0] == output || g.inputs[1] == output).collect::<Vec<_>>();
            if let Some(g) = gates.iter().find(|g| g.output.starts_with('z')) {
                Some(format!("z{:02}", parse_usize(&g.output[1..]).unwrap() - 1))
            } else {
                gates.iter().find_map(|g| first_z_that_should_use(machine, g.output))
            }
        }

        // It seems in the data, a few output wires are swapped with carry wires.
        // As such, we find the gates outputting to a z-wire with a wrong operation type and carry gates with an XOR
        let wrong_z_gates = self.gates.iter()
            .filter(|g| g.output.starts_with("z") && g.output.ne("z45") && g.operation != Operation::XOR)
            .collect::<Vec<_>>();
        let wrong_carry_gates = self.gates.iter()
            .filter(|g| !g.inputs[0].starts_with(['x', 'y']) && !g.inputs[1].starts_with(['x', 'y']) && !g.output.starts_with('z') && g.operation == Operation::XOR)
            .collect::<Vec<_>>();

        // println!("Wrong z-outs: {:?}", wrong_z_gates.iter().map(|c| c.output).collect::<Vec<_>>());
        // println!("Wrong carries: {:?}", wrong_carry_gates.iter().map(|c| c.output).collect::<Vec<_>>());

        let mut swap_pairs = vec![];

        // To get the swapped pair, we need to find where the carry gate should've outputted its value (a z before the one using it as carry)
        for wrong_carry in wrong_carry_gates {
            // Find a z-out that uses this carry's output, then find the wrong z gate
            let target_z = first_z_that_should_use(self, wrong_carry.output).unwrap();
            let swap_z_gate = wrong_z_gates.iter().find(|g| g.output == target_z).unwrap();
            swap_pairs.push([wrong_carry.output, swap_z_gate.output]);
        }

        // println!("Swaps to fix output/carries: {:?}", swap_pairs);

        let mut test_clone = self.clone();

        // Handle swaps
        for gate in test_clone.gates.iter_mut() {
            if let Some(new_output) = swap_pairs.iter().find_map(|p| if p[0] == gate.output { Some(p[1]) } else if p[1] == gate.output { Some(p[0]) } else { None }) {
                gate.output = new_output;
            }
        }

        test_clone.process_inputs();
        let new_result = test_clone.get_result();

        let difference = new_result.bitxor(z);
        // println!("After fixing, we had {} diff ({} trailing zeroes)", difference, difference.trailing_zeros());

        // Finally, we find 3 pairs, but there is still a swap between a carry and output gate, we can find that
        // by counting the zeroes at the end of our difference, which corresponds to the x and y input wires that
        // go the wrong way:
        let mismapped_input = difference.trailing_zeros();
        let [first, second] = self.gates.iter().filter(|g| g.inputs[0].ends_with(&mismapped_input.to_string()) && g.inputs[1].ends_with(&mismapped_input.to_string())).collect::<Vec<_>>()[..] else { panic!("Assumption failed") };

        let mut wrong_wires = vec![first.output, second.output];
        // println!("Crossed carries: {:?}", wrong_wires);
        for [left, right] in swap_pairs {
            wrong_wires.push(left);
            wrong_wires.push(right);
        }
        wrong_wires.sort();
        wrong_wires
    }
}

impl <'a> Wire<'a> {
    fn parse(input: &'a str) -> Result<Wire<'a>, String> {
        let [name, value] = input.split(": ").collect::<Vec<_>>()[..] else { return Err(format!("Invalid wire '{}'", input)) };

        Ok(Self {
            name, value: parse_u8(value)?
        })
    }
}

impl Operation {
    fn apply(&self, inputs: [&u8; 2]) -> u8 {
        match self {
            Operation::XOR => if inputs[0] != inputs[1] { 1 } else { 0 },
            Operation::OR => if inputs[0] == &1 || inputs[1] == &1 { 1 } else { 0 },
            Operation::AND => if inputs[0] == &1 && inputs[1] == &1 { 1 } else { 0 },
        }
    }
}

impl <'a> Gate<'a> {
    fn parse(input: &'a str) -> Result<Gate<'a>, String> {
        let [input_a, operation, input_b, "->", output] = input.split(" ").collect::<Vec<_>>()[..] else { return Err(format!("Invalid gate '{}'", input)) };

        Ok(Self {
            inputs: [input_a, input_b],
            operation: operation.parse()?,
            output
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::days::day24::{Gate, Machine, Operation};

    #[test]
    fn test_parse() {
        let machine_result = Machine::parse(TEST_INPUT);
        assert!(machine_result.is_ok());

        let machine = machine_result.unwrap();
        assert_eq!(machine.wires.get("x00"), Some(&1));
        assert_eq!(machine.wires.get("x01"), Some(&0));
        assert_eq!(machine.gates.len(), 36);

        let small_machine = Machine::parse(SMALL_TEST_INPUT).unwrap();
        assert_eq!(small_machine.wires, HashMap::from([
            ("x00", 1),
            ("x01", 1),
            ("x02", 1),
            ("y00", 0),
            ("y01", 1),
            ("y02", 0)
        ]));
        assert_eq!(small_machine.gates, vec![
            Gate { inputs: ["x00", "y00"], operation: Operation::AND, output: "z00" },
            Gate { inputs: ["x01", "y01"], operation: Operation::XOR, output: "z01" },
            Gate { inputs: ["x02", "y02"], operation: Operation::OR, output: "z02" },
        ])
    }

    #[test]
    fn test_process_inputs() {
        let mut small_machine = Machine::parse(SMALL_TEST_INPUT).unwrap();

        small_machine.process_inputs();

        assert_eq!(small_machine.wires["z00"], 0);
        assert_eq!(small_machine.wires["z01"], 0);
        assert_eq!(small_machine.wires["z02"], 1);
        assert_eq!(small_machine.get_result(), 4);

        let mut machine = Machine::parse(TEST_INPUT).unwrap();
        machine.process_inputs();

        assert_eq!(machine.wires["z00"], 0);
        assert_eq!(machine.wires["z01"], 0);
        assert_eq!(machine.wires["z02"], 0);
        assert_eq!(machine.wires["z03"], 1);
        assert_eq!(machine.wires["z04"], 0);
        assert_eq!(machine.wires["z05"], 1);
        assert_eq!(machine.wires["z06"], 1);
        assert_eq!(machine.wires["z07"], 1);
        assert_eq!(machine.wires["z08"], 1);
        assert_eq!(machine.wires["z09"], 1);
        assert_eq!(machine.wires["z10"], 1);
        assert_eq!(machine.wires["z11"], 0);
        assert_eq!(machine.wires["z12"], 0);
        assert_eq!(machine.get_result(), 2024);
    }

    const SMALL_TEST_INPUT: &str = "\
        x00: 1\n\
        x01: 1\n\
        x02: 1\n\
        y00: 0\n\
        y01: 1\n\
        y02: 0\n\
        \n\
        x00 AND y00 -> z00\n\
        x01 XOR y01 -> z01\n\
        x02 OR y02 -> z02\n\
    ";

    const TEST_INPUT: &str = "\
        x00: 1\n\
        x01: 0\n\
        x02: 1\n\
        x03: 1\n\
        x04: 0\n\
        y00: 1\n\
        y01: 1\n\
        y02: 1\n\
        y03: 1\n\
        y04: 1\n\
        \n\
        ntg XOR fgs -> mjb\n\
        y02 OR x01 -> tnw\n\
        kwq OR kpj -> z05\n\
        x00 OR x03 -> fst\n\
        tgd XOR rvg -> z01\n\
        vdt OR tnw -> bfw\n\
        bfw AND frj -> z10\n\
        ffh OR nrd -> bqk\n\
        y00 AND y03 -> djm\n\
        y03 OR y00 -> psh\n\
        bqk OR frj -> z08\n\
        tnw OR fst -> frj\n\
        gnj AND tgd -> z11\n\
        bfw XOR mjb -> z00\n\
        x03 OR x00 -> vdt\n\
        gnj AND wpb -> z02\n\
        x04 AND y00 -> kjc\n\
        djm OR pbm -> qhw\n\
        nrd AND vdt -> hwm\n\
        kjc AND fst -> rvg\n\
        y04 OR y02 -> fgs\n\
        y01 AND x02 -> pbm\n\
        ntg OR kjc -> kwq\n\
        psh XOR fgs -> tgd\n\
        qhw XOR tgd -> z09\n\
        pbm OR djm -> kpj\n\
        x03 XOR y03 -> ffh\n\
        x00 XOR y04 -> ntg\n\
        bfw OR bqk -> z06\n\
        nrd XOR fgs -> wpb\n\
        frj XOR qhw -> z04\n\
        bqk OR frj -> z07\n\
        y03 OR x01 -> nrd\n\
        hwm AND bqk -> z03\n\
        tgd XOR rvg -> z12\n\
        tnw OR pbm -> gnj\n\
    ";
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "XOR" => Ok(Operation::XOR),
            "OR" => Ok(Operation::OR),
            "AND" => Ok(Operation::AND),
            _ => Err(format!("Unknown operation '{}'", s))
        }
    }
}