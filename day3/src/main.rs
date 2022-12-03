use itertools::Itertools;

fn divide_pack(backpack: &str) -> (&str, &str) {
    backpack.split_at(backpack.len() / 2)
}

#[test]
fn test_divide_pack() {
    assert_eq!(
        divide_pack("vJrwpWtwJgWrhcsFMMfFFhFp"),
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
    );
    assert_eq!(
        divide_pack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
    );
}

fn find_duplicate(backpack: &str) -> char {
    let (comp1, comp2) = divide_pack(backpack);

    for c in comp1.chars() {
        if comp2.contains(c) {
            return c;
        }
    }

    panic!("invalid backpack");
}

#[test]
fn test_find_duplicate() {
    assert_eq!(find_duplicate("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    assert_eq!(find_duplicate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
}

fn char_to_score(c: char) -> u64 {
    if c.is_lowercase() {
        c as u64 - ('a' as u64) + 1
    } else {
        c as u64 - ('A' as u64) + 27
    }
}

#[test]
fn test_char_to_score() {
    assert_eq!(char_to_score('a'), 1);
    assert_eq!(char_to_score('C'), 29);
}

fn get_badge(group: &Vec<&str>) -> char {
    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            return c;
        }
    }

    panic!("invalid group");
}

#[test]
fn test_get_badge() {
    assert_eq!(
        get_badge(&vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        ]),
        'r'
    );
    assert_eq!(
        get_badge(&vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ]),
        'Z'
    );
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = input
        .lines()
        .map(|x| find_duplicate(x))
        .map(|x| char_to_score(x))
        .sum::<u64>();
    println!("answer 1: {}", answer1);

    let answer2 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| get_badge(&chunk.collect::<Vec<_>>()))
        .map(|c| char_to_score(c))
        .sum::<u64>();
    println!("answer 2: {}", answer2);
}
