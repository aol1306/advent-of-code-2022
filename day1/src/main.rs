fn main() {
    // process input
    let mut elves_calories = vec![0];
    for line in include_str!("input.txt").lines() {
        match line {
            "" => elves_calories.push(0),
            v => {
                let i = elves_calories.len() - 1;
                elves_calories[i] += v.parse::<u64>().unwrap();
            }
        };
    }

    // find max
    let answer1 = elves_calories.iter().max().unwrap();
    println!("answer 1: {}", answer1);

    // find sum of three max
    elves_calories.sort();
    let answer2: u64 = elves_calories.iter().rev().take(3).sum();
    println!("answer 2: {}", answer2);
}
