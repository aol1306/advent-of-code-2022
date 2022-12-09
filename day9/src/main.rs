/*
  ......
  ......
  .TH...
↑ ......
y s.....
  x→
*/
#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq)]
struct Movement {
    direction: Direction,
}

fn parse_input(input: &str) -> Vec<Movement> {
    vec![]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "R 1\nU 2";
        let expected = vec![Movement{direction: Direction::Right}, Movement{direction: Direction::Up}, Movement{direction: Direction::Up}];

        assert_eq!(parse_input(input), expected);
    }
}