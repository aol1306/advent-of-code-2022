use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

#[derive(Debug)]
struct Storage {
    stacks: Vec<Stack>,
}

impl TryFrom<&str> for Storage {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut stacks: Vec<Stack> = vec![];

        let stack_count = (value
            .lines()
            .take(1)
            .map(|x| x.chars().count())
            .collect::<Vec<_>>()[0]
            + 1)
            / 4;

        for _ in 0..stack_count {
            stacks.push(Stack { crates: vec![] });
        }

        value.lines().rev().skip(1).for_each(|x| {
            x.chars().skip(1).step_by(4).enumerate().for_each(|x| {
                if x.1 != ' ' {
                    let c = Crate { id: x.1 };
                    stacks[x.0].crates.push(c);
                }
            })
        });

        Ok(Storage { stacks: stacks })
    }
}

impl Storage {
    fn execute_movement_instruction(&mut self, movement_instuction: &str) {
        let caps = RE.captures(movement_instuction).unwrap();
        let count = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap() - 1;
        let to = caps[3].parse::<usize>().unwrap() - 1;

        self.move_crates(count, from, to);
    }

    fn move_crates(&mut self, count: usize, from: usize, to: usize) {
        for _ in 0..count {
            self.move_crate(from, to);
        }
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        let moved_crate = self.stacks[from].crates.pop().unwrap();
        self.stacks[to].crates.push(moved_crate);
    }

    fn execute_movement_instruction_9001(&mut self, movement_instuction: &str) {
        let caps = RE.captures(movement_instuction).unwrap();
        let count = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap() - 1;
        let to = caps[3].parse::<usize>().unwrap() - 1;

        self.move_crates_9001(count, from, to);
    }

    fn move_crates_9001(&mut self, count: usize, from: usize, to: usize) {
        let range = self.stacks[from].crates.len() - count..;
        // remove from old stack
        let mut moved_crates = self.stacks[from].crates.drain(range).collect::<Vec<_>>();
        // add to new stack
        self.stacks[to].crates.append(&mut moved_crates);
    }

    fn get_top_crates(&self) -> Vec<Crate> {
        let mut ret = vec![];
        for stack in &self.stacks {
            ret.push(stack.crates.last().copied().unwrap());
        }
        ret
    }
}

#[derive(Debug)]
struct Stack {
    crates: Vec<Crate>,
}

#[derive(Debug, Clone, Copy)]
struct Crate {
    id: char,
}

fn main() {
    let input = include_str!("input.txt");
    let input_parts = input.split("\n\n").collect::<Vec<_>>();
    let storage = input_parts[0];
    let movements = input_parts[1];

    // part 1
    {
        let mut storage = Storage::try_from(storage).unwrap();
        for line in movements.lines() {
            storage.execute_movement_instruction(line);
        }

        println!(
            "answer 1: {}",
            storage
                .get_top_crates()
                .iter()
                .fold(String::new(), |acc, x| acc + &x.id.to_string())
        );
    }
    // part 2
    {
        let mut storage = Storage::try_from(storage).unwrap();
        for line in movements.lines() {
            storage.execute_movement_instruction_9001(line);
        }

        println!(
            "answer 2: {}",
            storage
                .get_top_crates()
                .iter()
                .fold(String::new(), |acc, x| acc + &x.id.to_string())
        );
    }
}
