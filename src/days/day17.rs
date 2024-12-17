use std::collections::VecDeque;
use std::ops::BitXor;
use std::str::FromStr;
use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::parser::Parser;

pub const DAY17: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let program: Program = input.parse().unwrap();

    let output = program.run();
    println!("Program output: {}", output.map(|v| v.to_string()).join(","));
}

fn puzzle2(input: &String) {
    let program: Program = input.parse().unwrap();

    let output = program.find_quine_value();
    println!("Reg A value for quine: {}", output.unwrap());
}

struct Program {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,

    program: Vec<usize>,
}

impl Program {
    fn run(&self) -> Vec<usize> {
        self.run_internal(self.reg_a)
    }

    fn run_internal(&self, reg_a_init: usize) -> Vec<usize> {
        let mut reg_a = reg_a_init;
        let mut reg_b = self.reg_b;
        let mut reg_c = self.reg_c;

        let mut instruction_pointer = 0;
        let mut output = vec![];

        while instruction_pointer + 1 < self.program.len() {
            let operation = self.program[instruction_pointer];
            let literal_operand = self.program[instruction_pointer + 1];

            let combo_operand = match literal_operand {
                0..=3 => Some(literal_operand),
                4 => Some(reg_a),
                5 => Some(reg_b),
                6 => Some(reg_c),
                _ => None
            };

            match operation {
                0 => {
                    // adv, divide value in reg_a by 2^combo
                    reg_a = reg_a / 2_usize.pow(combo_operand.unwrap() as u32);
                },
                1 => {
                    // bxl, bitwise XOR of reg_b and the literal operand
                    reg_b = reg_b.bitxor(literal_operand);
                },
                2 => {
                    // bst, combo % 8 => reg_b
                    reg_b = combo_operand.unwrap() % 8;
                },
                3 => {
                    // jnz, if reg_a = 0, continue, else jump to literal
                    if reg_a != 0 {
                        instruction_pointer = literal_operand;
                        continue; // skip default increment
                    }
                },
                4 => {
                    // bxc, bitwise XOR of reg_b and reg_c => reg_b (ignore operand)
                    reg_b = reg_b.bitxor(reg_c);
                },
                5 => {
                    // out, combo % 8 => output
                    output.push(combo_operand.unwrap() % 8);
                },
                6 => {
                    // bdv, reg_a / 2^combo => reg_b
                    reg_b = reg_a / 2_usize.pow(combo_operand.unwrap() as u32);
                },
                7 => {
                    // cdv, reg_a / 2^combo => reg_c
                    reg_c = reg_a / 2_usize.pow(combo_operand.unwrap() as u32);
                },
                _ => unreachable!("Invalid opcode {}", operation)
            }

            instruction_pointer += 2;
        }

        output
    }

    fn find_quine_value(&self) -> Option<usize> {
        // Base on my input, which divides reg_a by 8 every loop, we first check which value(s)
        // between 1..8 output a single digit matching the last program value.
        // Taking that number, we multiply by 8**(program length), and validate output

        // To make it work for the tests as well, we find the ADV instruction (which indeed assumes
        // based on the test and my actual input there is just one which uses a 1..3 value)
        let divisor = (0..self.program.len()).step_by(2).find_map(|idx| {
            if self.program[idx] == 0 && (1..=3).contains(&self.program[idx+1]) {
                Some(2_usize.pow(self.program[idx+1] as u32))
            } else {
                None
            }
        }).unwrap();

        println!("Program divides a by {} every loop", divisor);

        // To end up at 0 in the last, run, the initial a value cannot be more than the a_divisor,
        // it obviously also cannot be 0.
        let mut queue = VecDeque::new();
        queue.push_back((1..divisor, 1));

        while let Some((range, step)) = queue.pop_front() {
            let required = &self.program[self.program.len() - step..];

            for a in range {
                let output = self.run_internal(a);

                // if we find a value for a that works on the last number, we can back-track for the previous value.
                // Since we divide with truncation, the next value of a can be anywhere between (a*divisor)..(a*(divisor+1))
                // We might need a few end numbers to find the right one though :)
                if output == required {
                    if step == self.program.len() {
                        return Some(a); // We got the whole program!
                    }

                    let min = a * divisor;
                    let max = min + divisor;

                    queue.push_back((min..max, step+1));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day17::Program;

    const TEST_INPUT: &str = "\
        Register A: 729\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,1,5,4,3,0\n\
    ";

    const QUINE_INPUT: &str = "\
        Register A: 2024\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,3,5,4,3,0\n\
    ";

    #[test]
    fn test_program_run() {
        let program: Program = TEST_INPUT.parse().unwrap();

        let output = program.run();

        assert_eq!(output, vec![4,6,3,5,6,3,5,2,1,0]);
    }

    #[test]
    fn test_find_quine_value() {
        let quine: Program = QUINE_INPUT.parse().unwrap();

        let quine_value = quine.find_quine_value();
        assert_eq!(quine_value, Some(117440));
    }
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);

        parser.literal("Register A:")?;
        let reg_a = parser.usize()?;
        parser.literal("Register B:")?;
        let reg_b = parser.usize()?;
        parser.literal("Register C:")?;
        let reg_c = parser.usize()?;

        parser.literal("Program:")?;
        let mut program = vec![];
        program.push(parser.usize()?);

        while !parser.is_exhausted() {
            parser.literal(",")?;
            program.push(parser.usize()?);
        }

        Ok(Self { reg_a, reg_b, reg_c, program })
    }
}