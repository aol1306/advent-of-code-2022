use day12::*;

fn main() {
    let grid = parse_input(include_str!("input.txt"));
    // answer 1: 339
    println!("answer 1: {}", grid.part1());
    // answer 2: 332
    println!("answer 2: {}", grid.part2());
}
