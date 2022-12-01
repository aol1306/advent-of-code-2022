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

/*
 * 1. how to create a stdin interator, that reads input line by line?
 * use std::io::{self, BufRead};
 * let iter = io::stdin().lock().lines();
 *
 * 2. how to check variable x type using a compiler error (trick)?
 * () = x;
 *
 * 3. how to match a String against string literals?
 * convert the String to &str using .as_str()
 *
 * 4. convert String s to u64 - assign the result to x: u64
 * let x: u64 = s.parse::<u64>().unwrap();
 *
 * 5. how to find a max value in an iterator?
 * iter.max();
 *
 * 6. how to stable sort a mutable Vec?
 * vec.sort(); // sorts from min to max
 *
 * 7. how to reverse an iterator?
 * iter.rev();
 *
 * 8. how to take n elements of iterator (create an iterator that only iterates over the first n iterations of iter)
 * iter.take(n);
 *
 * 9. how to sum an iterator? (take each element, add them together, and return the result) (save result to v: u64)
 * let v: u64 = iter.sum();
 *
 * 10. how to include a UTF-8 encoded file as a str?
 * include_str!("path");
 *
 * 11. how to create an iterator over the lines of a string s?
 * s.lines();
 */
