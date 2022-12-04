use std::time::Instant;

#[derive(Debug)]
struct AssignmentPair {
    elf1: Assignment,
    elf2: Assignment,
}

impl TryFrom<&str> for AssignmentPair {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vals = value.split(",").collect::<Vec<_>>();

        Ok(AssignmentPair {
            elf1: Assignment::try_from(*vals.get(0).ok_or("invalid input")?)?,
            elf2: Assignment::try_from(*vals.get(1).ok_or("invalid input")?)?,
        })
    }
}

impl AssignmentPair {
    fn is_self_contained(&self) -> bool {
        self.elf1.contains(&self.elf2) || self.elf2.contains(&self.elf1)
    }

    fn is_overlapping(&self) -> bool {
        self.elf1.overlaps(&self.elf2)
    }
}

#[derive(Debug)]
struct Assignment {
    start: u64,
    end: u64,
}

impl TryFrom<&str> for Assignment {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vals = value
            .split("-")
            .filter_map(|x| x.parse::<u64>().ok())
            .collect::<Vec<_>>();

        Ok(Assignment {
            start: *vals.get(0).ok_or("invalid input")?,
            end: *vals.get(1).ok_or("invalid input")?,
        })
    }
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        (self.end >= other.start && self.start <= other.end)
            || (other.end <= self.start && other.start >= self.end)
    }
}

fn main() {
    let input = include_str!("input.txt");

    let start = Instant::now();
    let part1 = input
        .lines()
        .map(|line| AssignmentPair::try_from(line).unwrap())
        .map(|pair| pair.is_self_contained())
        .filter(|result| *result)
        .count();

    println!("answer 1: {part1} {:?}", start.elapsed());

    let start = Instant::now();
    let part2 = input
        .lines()
        .map(|line| AssignmentPair::try_from(line).unwrap())
        .map(|pair| pair.is_overlapping())
        .filter(|result| *result)
        .count();

    println!("answer 2: {part2} {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_assignment_contains() {
        let a = Assignment::try_from("2-8").unwrap();
        let b = Assignment::try_from("3-7").unwrap();
        assert_eq!(a.contains(&b), true);
        assert_eq!(b.contains(&a), false);
    }

    #[test]
    fn test_assignment_overlaps() {
        let a = Assignment::try_from("5-7").unwrap();
        let b = Assignment::try_from("7-9").unwrap();
        assert_eq!(a.overlaps(&b), true);
        let a = Assignment::try_from("2-4").unwrap();
        let b = Assignment::try_from("6-8").unwrap();
        assert_eq!(a.overlaps(&b), false);
    }

    #[test]
    fn test_assignment_pair_is_self_contained() {
        let p = AssignmentPair::try_from("6-6,4-6").unwrap();
        assert_eq!(p.is_self_contained(), true);
        let p = AssignmentPair::try_from("2-3,4-5").unwrap();
        assert_eq!(p.is_self_contained(), false);
    }

    #[test]
    fn test_assignment_pair_is_overlapping() {
        let p = AssignmentPair::try_from("2-3,4-5").unwrap();
        assert_eq!(p.is_overlapping(), false);
        let p = AssignmentPair::try_from("2-6,4-8").unwrap();
        assert_eq!(p.is_overlapping(), true);

        let p = AssignmentPair::try_from("3-4,1-2").unwrap();
        assert_eq!(p.is_overlapping(), false);
    }
}
