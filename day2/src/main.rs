use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
static ref WIN_TABLE: HashMap<&'static str, &'static str> = {
    let mut m = HashMap::new();
    m.insert("C", "A"); // to defeat C, choose A (rock defeats scissors)
    m.insert("B", "C");
    m.insert("A", "B");
    m
};

static ref LOSE_TABLE: HashMap<&'static str, &'static str> = {
    let mut m = HashMap::new();
    m.insert("A", "C"); // to lose to A, choose C (rock defeats scissors)
    m.insert("C", "B");
    m.insert("B", "A");
    m
};
}

// calculate score for a rock-paper-scissors game
fn calculate_score(round_shapes: &(&str, &str)) -> u64 {
    // points for chosen shape
    let mut score = match round_shapes.1 {
        "A" => 1, // rock
        "B" => 2, // paper
        "C" => 3, // scissors
        _ => panic!(),
    };
    // points for result - only winning combinations
    if WIN_TABLE.get(round_shapes.0).unwrap() == &round_shapes.1 {
        score += 6;
    }
    // points for a draw
    if round_shapes.0 == round_shapes.1 {
        score += 3;
    }
    score
}

#[test]
fn test_calculate_score() {
    assert_eq!(calculate_score(&("A", "B")), 8);
    assert_eq!(calculate_score(&("B", "A")), 1);
    assert_eq!(calculate_score(&("C", "C")), 6);
}

// part 1 - assume X = A, Y = B, Z = C
fn calculate_round_shapes_1<'a>(round: &'a (&'a str, &'a str)) -> (&'a str, &'a str) {
    match round.1 {
        "X" => (round.0, "A"),
        "Y" => (round.0, "B"),
        "Z" => (round.0, "C"),
        _ => panic!(),
    }
}

#[test]
fn test_calculate_round_shapes_1() {
    assert_eq!(calculate_round_shapes_1(&("A", "X")), ("A", "A"));
}

// part 2 - X -> we have to lose, Y -> draw, Z -> win
fn calculate_round_shapes_2<'a>(round: &'a (&'a str, &'a str)) -> (&'a str, &'a str) {
    match round.1 {
        // lose - choose losing shape
        "X" => (round.0, LOSE_TABLE.get(round.0).unwrap()),
        // draw - choose the same shape as the opponent
        "Y" => (round.0, round.0),
        // win - choose winning shape
        "Z" => (round.0, WIN_TABLE.get(round.0).unwrap()),
        _ => panic!(),
    }
}

#[test]
fn test_calculate_round_shapes_2() {
    assert_eq!(calculate_round_shapes_2(&("A", "X")), ("A", "C"));
    assert_eq!(calculate_round_shapes_2(&("B", "Y")), ("B", "B"));
    assert_eq!(calculate_round_shapes_2(&("C", "Z")), ("C", "A"));
}

fn main() {
    let input = include_str!("input.txt");
    let rounds = input
        .lines() // one line - one round
        .map(|x| {
            // convert to pairs
            let mut s = x.split(" ");
            (s.next().unwrap(), s.next().unwrap())
        })
        .collect::<Vec<_>>();

    let total: u64 = rounds
        .iter()
        .map(|x| calculate_score(&calculate_round_shapes_1(x)))
        .sum();

    println!("answer 1: {total}");

    let total2: u64 = rounds
        .iter()
        .map(|x| calculate_score(&calculate_round_shapes_2(x)))
        .sum();

    println!("answer 2: {total2}");
}
