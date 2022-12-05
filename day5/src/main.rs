use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn execute_movement_instruction(&mut self, movement_instruction: &str) {
        let caps = RE.captures(movement_instruction).unwrap();
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

    fn execute_movement_instruction_9001(&mut self, movement_instruction: &str) {
        let caps = RE.captures(movement_instruction).unwrap();
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

    fn top_crates_pretty(&self) -> String {
        self.get_top_crates()
            .iter()
            .fold(String::new(), |acc, x| acc + &x.id.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stack {
    crates: Vec<Crate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        println!("answer 1: {}", storage.top_crates_pretty());
    }
    // part 2
    {
        let mut storage = Storage::try_from(storage).unwrap();
        for line in movements.lines() {
            storage.execute_movement_instruction_9001(line);
        }

        println!("answer 2: {}", storage.top_crates_pretty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    lazy_static! {
        static ref TEST_STORAGE: Storage = Storage {
            stacks: vec![
                Stack {
                    crates: vec![Crate { id: 'Z' }, Crate { id: 'N' }],
                },
                Stack {
                    crates: vec![Crate { id: 'M' }, Crate { id: 'C' }, Crate { id: 'D' }],
                },
                Stack {
                    crates: vec![Crate { id: 'P' }],
                },
            ],
        };
    }

    #[test]
    fn test_storage_from() {
        let test_str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        println!("{}", test_str);

        assert_eq!(Storage::try_from(test_str).unwrap(), *TEST_STORAGE);
    }

    #[test]
    fn test_storage_execute_movement() {
        let mut storage = TEST_STORAGE.clone();
        storage.execute_movement_instruction("move 2 from 2 to 3");
        assert_eq!(storage.stacks[2].crates.last().unwrap(), &Crate{id: 'C'});
    }

    #[test]
    fn test_storage_execute_movement_9001() {
        let mut storage = TEST_STORAGE.clone();
        storage.execute_movement_instruction_9001("move 2 from 1 to 3");
        assert_eq!(storage.stacks[2].crates.last().unwrap(), &Crate{id: 'N'});
    }

    #[test]
    fn test_storage_top_crates_pretty() {
        assert_eq!(TEST_STORAGE.top_crates_pretty(), "NDP");
    }

    #[test]
    fn test_all() {
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

            assert_eq!("RLFNRTNFB", storage.top_crates_pretty());
        }
        // part 2
        {
            let mut storage = Storage::try_from(storage).unwrap();
            for line in movements.lines() {
                storage.execute_movement_instruction_9001(line);
            }

            assert_eq!("MHQTLJRLB", storage.top_crates_pretty());
        }
    }
}
