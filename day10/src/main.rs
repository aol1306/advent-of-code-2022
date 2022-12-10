use crate::cpu::{Cpu, Instruction};
use std::collections::VecDeque;

mod cpu {
    use std::collections::VecDeque;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Instruction {
        Noop,
        Addx(i32),
    }

    #[derive(Debug)]
    pub struct Cpu {
        register_x: i32,
        cycle_count: i32,
        instruction_buf: VecDeque<Instruction>,

        currently_processing: Option<Instruction>,
        remaining_processing: usize,
    }

    impl Cpu {
        pub fn new(instructions: VecDeque<Instruction>) -> Cpu {
            Cpu {
                register_x: 1,
                cycle_count: 0,
                instruction_buf: instructions,
                currently_processing: None,
                remaining_processing: 0,
            }
        }

        fn execute_instruction(&mut self, instruction: &Instruction) {
            match instruction {
                Instruction::Noop => {}
                Instruction::Addx(n) => self.register_x += n,
            }
        }

        pub fn multi_tick(&mut self, count: usize) {
            for _ in 0..count {
                self.tick();
            }
        }

        pub fn tick(&mut self) {
            // EXECUTE
            // if there is an instruction in currently_processing, decrease remaining_processing
            if let Some(instruction) = self.currently_processing {
                self.remaining_processing -= 1;
                // if its zero, do the instruction action and remove the instruction from currently_processing
                if self.remaining_processing == 0 {
                    self.execute_instruction(&instruction);
                    self.currently_processing = None;
                }
            }

            // LOAD
            // if there is no instruction in currently_processing, put instruction in currently_processing
            //    and put its cost in remaining_processing
            if self.currently_processing.is_none() {
                self.currently_processing = self.instruction_buf.pop_front();
                self.remaining_processing = match self.currently_processing {
                    Some(Instruction::Addx(_)) => 2,
                    Some(Instruction::Noop) => 1,
                    None => 0,
                }
            }

            // increase cycle count
            self.cycle_count += 1;
        }

        pub fn get_signal_strength(&self) -> i32 {
            self.cycle_count * self.register_x
        }
    }
}

fn parse_input(input: &str) -> VecDeque<Instruction> {
    input
        .lines()
        .map(|x| match x {
            "noop" => Instruction::Noop,
            x => match x.split(' ').collect::<Vec<_>>().as_slice() {
                ["addx", n] => Instruction::Addx(n.parse().unwrap()),
                _ => unimplemented!(),
            },
        })
        .collect()
}

fn part1(input: &str) {
    let instructions = parse_input(input);
    let mut cpu = Cpu::new(instructions);

    let mut total_strength = 0;

    cpu.multi_tick(20);
    total_strength += cpu.get_signal_strength();
    for _ in 1..6 {
        cpu.multi_tick(40);
        total_strength += cpu.get_signal_strength();
    }

    println!("answer 1: {}", total_strength);
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let instructions = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        assert_eq!(parse_input("noop\naddx 3\naddx -5"), instructions);
    }

    #[test]
    fn test_tick() {
        let mut instructions = VecDeque::new();
        instructions.push_back(Instruction::Noop);
        instructions.push_back(Instruction::Addx(3));
        let mut cpu = Cpu::new(instructions);

        cpu.tick();
        assert_eq!(cpu.get_signal_strength(), 1); // x = 1, tick = 1
        cpu.tick();
        assert_eq!(cpu.get_signal_strength(), 2); // x = 1, tick = 2
        cpu.tick();
        assert_eq!(cpu.get_signal_strength(), 3); // x = 1, tick = 3
        cpu.tick();
        assert_eq!(cpu.get_signal_strength(), 16); // x = 4, tick = 4
    }

    #[test]
    fn test_example_input() {
        let input = include_str!("test.txt");
        let instructions = parse_input(input);
        let mut cpu = Cpu::new(instructions);
        cpu.multi_tick(20);
        assert_eq!(cpu.get_signal_strength(), 420);
        cpu.multi_tick(40);
        assert_eq!(cpu.get_signal_strength(), 1140);
        cpu.multi_tick(40);
        assert_eq!(cpu.get_signal_strength(), 1800);
        cpu.multi_tick(40);
        assert_eq!(cpu.get_signal_strength(), 2940);
        cpu.multi_tick(40);
        assert_eq!(cpu.get_signal_strength(), 2880);
        cpu.multi_tick(40);
        assert_eq!(cpu.get_signal_strength(), 3960);
    }
}
