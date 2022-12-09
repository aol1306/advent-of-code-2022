use std::collections::HashSet;

/*
  ......
  ......
  .TH...
↑ ......
y s.....
  x→
*/
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn make_move(&mut self, movement: Movement) {
        match movement.direction {
            Direction::Up => self.x += 1,
            Direction::Down => self.x -= 1,
            Direction::Right => self.y += 1,
            Direction::Left => self.y -= 1,
        }
    }

    fn follow_position(&mut self, other: &Position) {
        // raise this flag if there was a movement caused by separation this round
        let mut moved_y = false;
        let mut moved_x = false;
        // movement only required if not touching other
        if self.is_touching(other) {
            return;
        }

        // need to move up
        if self.y + 1 < other.y {
            self.y += 1;
            moved_y = true;
        }
        // need to move down
        if other.y + 1 < self.y {
            self.y -= 1;
            moved_y = true;
        }
        // need to move right
        if self.x + 1 < other.x {
            self.x += 1;
            moved_x = true;
        }
        // need to move left
        if other.x + 1 < self.x {
            self.x -= 1;
            moved_x = true;
        }

        // diagonal adjustments
        if moved_x {
            // need to move up
            if self.y < other.y {
                self.y += 1;
            }
            // need to move down
            if other.y < self.y {
                self.y -= 1;
            }
        }
        if moved_y {
            // need to move right
            if self.x < other.x {
                self.x += 1;
            }
            // need to move left
            if other.x < self.x {
                self.x -= 1;
            }
        }
    }

    fn is_touching(&self, other: &Position) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Movement {
    direction: Direction,
}

fn parse_input(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|x| match x.split(' ').collect::<Vec<_>>().as_slice() {
            ["U", n] => std::iter::repeat(Movement {
                direction: Direction::Up,
            })
            .take(n.parse().unwrap()),
            ["D", n] => std::iter::repeat(Movement {
                direction: Direction::Down,
            })
            .take(n.parse().unwrap()),
            ["R", n] => std::iter::repeat(Movement {
                direction: Direction::Right,
            })
            .take(n.parse().unwrap()),
            ["L", n] => std::iter::repeat(Movement {
                direction: Direction::Left,
            })
            .take(n.parse().unwrap()),
            _ => unimplemented!(),
        })
        .flatten()
        .collect()
}

fn part1(movements: Vec<Movement>) -> usize {
    let mut head_position = Position { x: 1, y: 1 };
    let mut tail_position = Position { x: 1, y: 1 };
    let mut visited_by_tail: HashSet<Position> = HashSet::new();

    for movement in movements {
        head_position.make_move(movement);
        tail_position.follow_position(&head_position);
        visited_by_tail.insert(tail_position.clone());
    }

    visited_by_tail.len()
}

fn main() {
    let input = include_str!("input.txt");
    let movements = parse_input(input);
    println!("answer 1: {}", part1(movements));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "R 1\nU 2";
        let expected = vec![
            Movement {
                direction: Direction::Right,
            },
            Movement {
                direction: Direction::Up,
            },
            Movement {
                direction: Direction::Up,
            },
        ];

        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_is_touching() {
        // coverting
        assert_eq!(
            Position { x: 1, y: 1 }.is_touching(&Position { x: 1, y: 1 }),
            true
        );
        // to the right
        assert_eq!(
            Position { x: 1, y: 1 }.is_touching(&Position { x: 2, y: 1 }),
            true
        );
        // left-up
        assert_eq!(
            Position { x: 1, y: 1 }.is_touching(&Position { x: 0, y: 2 }),
            true
        );
        // not touching
        assert_eq!(
            Position { x: 1, y: 1 }.is_touching(&Position { x: 1, y: 3 }),
            false
        );
    }

    #[test]
    fn test_follow_position() {
        // do nothing when touching
        let mut p = Position { x: 1, y: 1 };
        p.follow_position(&Position { x: 1, y: 2 });
        assert_eq!(p, Position { x: 1, y: 1 });

        // move up when other is 2 above
        let mut p = Position { x: 1, y: 1 };
        p.follow_position(&Position { x: 1, y: 3 });
        assert_eq!(p, Position { x: 1, y: 2 });

        // move up-right when other is 2 above 1 right
        let mut p = Position { x: 1, y: 1 };
        p.follow_position(&Position { x: 2, y: 3 });
        assert_eq!(p, Position { x: 2, y: 2 });

        // move up-right when other is 1 above 2 right
        let mut p = Position { x: 1, y: 1 };
        p.follow_position(&Position { x: 3, y: 2 });
        assert_eq!(p, Position { x: 2, y: 2 });

        // move down-right when other is 1 down 2 right
        let mut p = Position { x: 1, y: 1 };
        p.follow_position(&Position { x: 3, y: 0 });
        assert_eq!(p, Position { x: 2, y: 0 });
    }
}
