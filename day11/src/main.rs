mod model;

use crate::model::Monkey;

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|m| Monkey::try_from(m).unwrap())
        .collect()
}

fn part1(mut monkeys: Vec<Monkey>) {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].inspect_all_items(|x| x / 3);
            for throw in throws {
                monkeys[throw.get_target_monkey()].accept_throw(&throw);
            }
        }
    }

    for monkey in &monkeys {
        println!("{}", monkey);
    }

    let mut activity_levels: Vec<_> = monkeys.iter().map(|m| m.get_inspect_count()).collect();
    activity_levels.sort_unstable();
    activity_levels.reverse();

    println!("answer 1: {}", activity_levels[0] * activity_levels[1]);
}

fn part2(mut monkeys: Vec<Monkey>) {
    let base = monkeys.iter().fold(1, |acc, x| acc * x.get_test());

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].inspect_all_items(|x| x % base);
            for throw in throws {
                monkeys[throw.get_target_monkey()].accept_throw(&throw);
            }
        }
    }

    for monkey in &monkeys {
        println!("{}", monkey);
    }

    let mut activity_levels: Vec<_> = monkeys.iter().map(|m| m.get_inspect_count()).collect();
    activity_levels.sort_unstable();
    activity_levels.reverse();

    println!("answer 2: {}", activity_levels[0] * activity_levels[1]);
}

fn main() {
    let input = include_str!("input.txt");

    let monkeys = parse_input(input);
    part1(monkeys);

    let monkeys = parse_input(input);
    part2(monkeys);
}
