use colored::Colorize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
struct Tree {
    height: i32,
    visible: bool,
    scenic_score: i32,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.visible {
            write!(f, "{}", self.height.to_string().green())
        } else {
            write!(f, "{}", self.height.to_string().red())
        }
    }
}

type Grid = Vec<Vec<Tree>>;

fn visualize(grid: &Grid) {
    for line in grid {
        for tree in line {
            print!("{}", tree);
        }
        println!();
    }
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|x| Tree {
                    height: x as i32,
                    visible: false,
                    scenic_score: 0,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part1(grid: &mut Grid) -> i32 {
    let mut visible = 0;
    // count how many trees are visible in each line
    // left to right
    for line in &mut *grid {
        let mut visibility_level: i32 = -1;
        for tree in &mut *line {
            if tree.height > visibility_level {
                visibility_level = tree.height;
                if !tree.visible {
                    tree.visible = true;
                    visible += 1;
                }
            }
        }
    }
    // right to left
    for line in &mut *grid {
        let mut visibility_level = -1;
        for tree in (&mut *line).into_iter().rev() {
            if tree.height > visibility_level {
                visibility_level = tree.height;
                if !tree.visible {
                    tree.visible = true;
                    visible += 1;
                }
            }
        }
    }
    // up to down
    for i in 0..grid[0].len() {
        let mut visibility_level: i32 = -1;
        for j in 0..grid.len() {
            let tree = &mut grid[j][i];
            if tree.height > visibility_level {
                visibility_level = tree.height;
                if !tree.visible {
                    tree.visible = true;
                    visible += 1;
                }
            }
        }
    }
    // down to up
    for i in 0..grid[0].len() {
        let mut visibility_level: i32 = -1;
        for j in (0..grid.len()).rev() {
            let tree = &mut grid[j][i];
            if tree.height > visibility_level {
                visibility_level = tree.height;
                if !tree.visible {
                    tree.visible = true;
                    visible += 1;
                }
            }
        }
    }

    visible
}

// calculate scenic scores
fn part2(grid: &mut Grid) -> i32 {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let tree = &grid[row][col];

            // go right
            let mut score_right = 0;
            for i in col + 1..grid.len() {
                let next_tree = &grid[row][i];
                score_right += 1;
                if next_tree.height >= tree.height {
                    break;
                }
            }

            // go left
            let mut score_left = 0;
            for i in (0..col).rev() {
                let next_tree = &grid[row][i];
                score_left += 1;
                if next_tree.height >= tree.height {
                    break;
                }
            }

            // go down
            let mut score_down = 0;
            for i in row + 1..grid[row].len() {
                let next_tree = &grid[i][col];
                score_down += 1;
                if next_tree.height >= tree.height {
                    break;
                }
            }

            // go up
            let mut score_up = 0;
            for i in (0..row).rev() {
                let next_tree = &grid[i][col];
                score_up += 1;
                if next_tree.height >= tree.height {
                    break;
                }
            }

            grid[row][col].scenic_score = score_left * score_right * score_down * score_up;
        }
    }

    grid.iter()
        .flatten()
        .map(|tree| tree.scenic_score)
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let mut grid = parse_input(input);
    let visible = part1(&mut grid);
    visualize(&grid);
    println!("answer 1: {}", visible);
    println!("answer 2: {}", part2(&mut grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT)[0][0],
            Tree {
                height: 3,
                visible: false,
                scenic_score: 0
            }
        );
    }

    #[test]
    fn test_part1() {
        let mut grid = parse_input(TEST_INPUT);
        assert_eq!(part1(&mut grid), 21);
    }

    #[test]
    fn test_part2() {
        let mut grid = parse_input(TEST_INPUT);
        part1(&mut grid);
        assert_eq!(part2(&mut grid), 8);
    }
}
