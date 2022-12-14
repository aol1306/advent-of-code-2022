use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Grid {
    points: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new() -> Self {
        Grid {
            points: vec![],
            width: 0,
            height: 0,
        }
    }

    fn xy_to_index(&self, coords: (usize, usize)) -> usize {
        coords.1 * self.width + coords.0
    }

    pub fn walkable_neighbours<F: Fn(char, char) -> bool>(
        &self,
        coords: (usize, usize),
        elevation_test: F,
    ) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        let elevation_now = self.get((coords.0, coords.1)).unwrap();
        // up, down, left, right
        let deltas: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for delta in deltas {
            // check for overflow
            if (coords.0 as isize) + delta.0 < 0 || (coords.1 as isize) + delta.1 < 0 {
                continue;
            }

            let coords = (
                ((coords.0 as isize) + delta.0) as usize,
                ((coords.1 as isize) + delta.1) as usize,
            );

            if let Some(elevation_next) = self.get(coords) {
                // replace S and E
                let elevation_now = match elevation_now {
                    'S' => 'a',
                    'E' => 'z',
                    _ => elevation_now,
                };
                let elevation_next = match elevation_next {
                    'S' => 'a',
                    'E' => 'z',
                    _ => elevation_next,
                };

                if elevation_test(elevation_now, elevation_next) {
                    ret.push(coords);
                }
            }
        }

        ret
    }

    fn elevation_test_part1(now: char, next: char) -> bool {
        (now as u8) + 1 >= next as u8
    }

    fn elevation_test_part2(now: char, next: char) -> bool {
        now as u8 <= (next as u8) + 1
    }

    pub fn get(&self, coords: (usize, usize)) -> Option<char> {
        self.points.get(self.xy_to_index(coords)).copied()
    }

    pub fn find(&self, c: char) -> Option<(usize, usize)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get((x, y)) == Some(c) {
                    return Some((x, y));
                }
            }
        }

        None
    }

    pub fn find_all(&self, c: char) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get((x, y)) == Some(c) {
                    ret.push((x, y));
                }
            }
        }
        ret
    }

    pub fn bfs<F: Fn(char) -> bool, G: Fn(char, char) -> bool>(
        &self,
        root: (usize, usize),
        goal: F,
        elevation_test: G,
    ) -> (HashMap<(usize, usize), (usize, usize)>, (usize, usize)) {
        let mut parent_child = HashMap::new();

        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut explored: HashSet<(usize, usize)> = HashSet::new();
        explored.insert(root);
        q.push_back(root);

        while let Some(v) = q.pop_front() {
            if goal(self.get(v).unwrap()) {
                return (parent_child, v);
            }
            for w in self.walkable_neighbours(v, &elevation_test) {
                if !explored.contains(&w) {
                    explored.insert(w);
                    parent_child.insert(w, v);
                    q.push_back(w);
                }
            }
        }

        panic!();
    }

    pub fn part1(&self) -> usize {
        let map = self.bfs(
            self.find('S').unwrap(),
            |x| x == 'E',
            Self::elevation_test_part1,
        );
        let mut path = vec![];
        let mut child = self.find('E').unwrap();
        // walk back the map from the goal
        while let Some(parent) = map.0.get(&child) {
            path.push(*parent);
            child = *parent;
        }
        path.len()
    }

    pub fn part2(&self) -> usize {
        let map = self.bfs(
            self.find('E').unwrap(),
            |x| x == 'S' || x == 'a',
            Self::elevation_test_part2,
        );
        let mut path = vec![];
        let mut child = map.1;
        // walk back the map from the goal
        while let Some(parent) = map.0.get(&child) {
            path.push(*parent);
            child = *parent;
        }
        path.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                ret += &self.get((x, y)).unwrap().to_string();
            }
            ret += "\n";
        }
        write!(f, "{ret}")
    }
}

pub fn parse_input(input: &str) -> Grid {
    let mut grid = Grid::new();

    for line in input.lines().map(|x| x.chars()) {
        grid.height += 1;
        for c in line {
            grid.points.push(c);
        }
    }
    grid.width = grid.points.len() / grid.height;

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test.txt");

    #[test]
    fn test_parse_input() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.width, 8);
    }

    #[test]
    fn test_get() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(grid.get((0, 0)), Some('S'));
        assert_eq!(grid.get((2, 2)), Some('c'));
        assert_eq!(grid.get((5, 2)), Some('E'));
    }

    #[test]
    fn test_find() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(grid.find('S'), Some((0, 0)));
        assert_eq!(grid.find('E'), Some((5, 2)));
    }

    #[test]
    fn test_find_all() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(
            grid.find_all('a'),
            vec![(1, 0), (0, 1), (0, 2), (0, 3), (0, 4)]
        );
    }

    #[test]
    fn test_walkable_neighbours() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(
            grid.walkable_neighbours((0, 0), Grid::elevation_test_part1),
            vec![(0, 1), (1, 0)]
        );
        assert_eq!(
            grid.walkable_neighbours((2, 2), Grid::elevation_test_part1),
            vec![(2, 1), (2, 3), (1, 2)]
        );
    }

    #[test]
    fn test_part1() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(grid.part1(), 31);
    }

    #[test]
    fn test_part2() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(grid.part2(), 29);
    }
}
