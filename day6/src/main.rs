use std::collections::{VecDeque, HashSet};
use std::time::Instant;

fn check_duplicates(v: &VecDeque<char>) -> bool {
    let mut set = HashSet::new();
    for c in v {
        if set.contains(c) {
            return true;
        }
        set.insert(c);
    }
    return false
}

fn find_seq_start(s: &str, contig_count: usize) -> usize {
    let mut buf: VecDeque<char> = VecDeque::new();

    for c in s.chars().enumerate() {
        // init
        if buf.len() < contig_count {
            buf.push_back(c.1);
        } else {
            if !check_duplicates(&buf) {
                return c.0;
            }
            buf.pop_front();
            buf.push_back(c.1);
        }
    }
    0
}

fn main() {
    let input = include_str!("input.txt").to_string();
    let input = input.trim();

    let start = Instant::now();
    // answer 1: 1109 109µs
    println!("answer 1: {} {:?}", find_seq_start(&input, 4), start.elapsed());
    let start = Instant::now();
    // answer 2: 3965 711.875µs
    println!("answer 2: {} {:?}", find_seq_start(&input, 14), start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_find_seq_start() {
        // size - 4
        assert_eq!(find_seq_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_seq_start("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_seq_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find_seq_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

        // size - 14
        assert_eq!(find_seq_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_seq_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_seq_start("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find_seq_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find_seq_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn test_check_duplicates() {
        let v1 = VecDeque::from(['a', 'b', 'c', 'd']);
        assert_eq!(check_duplicates(&v1), false);

        let v2 = VecDeque::from(['a', 'b', 'c', 'c']);
        assert_eq!(check_duplicates(&v2), true);

        let v3 = VecDeque::from(['a', 'a', 'c', 'c']);
        assert_eq!(check_duplicates(&v3), true);
    }
}
