use std::collections::HashMap;

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
    score += match round_shapes {
        ("C", "A") => 6, // rock defeats scissors
        ("B", "C") => 6, // scissors defeats paper
        ("A", "B") => 6, // paper defeats rock
        _ => 0,
    };
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
fn calculate_round_shapes_1<'a>(round: &'a(&'a str, &'a str)) -> (&'a str, &'a str) {
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
fn calculate_round_shapes_2<'a>(round: &'a(&'a str, &'a str)) -> (&'a str, &'a str) {
    // https://docs.rs/bimap/latest/bimap/
    let mut win_table = HashMap::new();
    win_table.insert("C", "A"); // to defeat C, choose A (rock defeats scissors)
    win_table.insert("B", "C");
    win_table.insert("A", "B");

    let mut lose_table = HashMap::new();
    lose_table.insert("A", "C"); // to lose to A, choose C (rock defeats scissors)
    lose_table.insert("C", "B");
    lose_table.insert("B", "A");

    match round.1 {
        // lose - choose 
        "X" => (round.0, lose_table.get(round.0).unwrap()),
        // draw - choose the same shape as the opponent
        "Y" => (round.0, round.0),
        // win - choose winning shape
        "Z" => (round.0, win_table.get(round.0).unwrap()),
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
            (
                s.next().unwrap(),
                s.next().unwrap(),
            )
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
