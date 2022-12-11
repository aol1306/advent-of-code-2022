mod model {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref MONKEY_RE: Regex = Regex::new(r"Monkey [0-9]+:\n.*: (?P<items>[0-9 ,]+)\n.*= (?P<a>old|[0-9]+) (?P<op>[\*+]) (?P<b>old|[0-9]+)\n.*y (?P<test>[0-9]+)\n.* (?P<action_true>[0-9]+)\n.* (?P<action_false>[0-9]+)").unwrap();
    }

    #[derive(Debug, PartialEq)]
    struct Item {
        worry_level: usize,
    }

    impl TryFrom<&str> for Item {
        type Error = &'static str;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            Ok(Item {
                worry_level: s.parse().map_err(|_| "error parsing item")?,
            })
        }
    }

    #[derive(Debug, PartialEq)]
    struct Monkey {
        items: Vec<Item>,
        operation: Operation,
        // divisible by
        test: usize,
        // monkey number if true
        action_true: usize,
        // monkey number if false
        action_false: usize,
    }

    impl TryFrom<&str> for Monkey {
        type Error = &'static str;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            let caps = MONKEY_RE.captures(s).ok_or("input does not match")?;
            let items: Vec<_> = caps
                .name("items")
                .unwrap()
                .as_str()
                .split(", ")
                .filter_map(|x| Item::try_from(x).ok())
                .collect();
            let a = OperationValue::try_from(caps.name("a").unwrap().as_str())?;
            let op = Operator::try_from(caps.name("op").unwrap().as_str())?;
            let b = OperationValue::try_from(caps.name("b").unwrap().as_str())?;
            let test: usize = caps.name("test").unwrap().as_str().parse().unwrap();
            let action_true: usize = caps.name("action_true").unwrap().as_str().parse().unwrap();
            let action_false: usize = caps.name("action_false").unwrap().as_str().parse().unwrap();

            Ok(Monkey {
                items,
                operation: Operation { a, op, b },
                test,
                action_true,
                action_false,
            })
        }
    }

    #[derive(Debug, PartialEq)]
    struct Operation {
        a: OperationValue,
        op: Operator,
        b: OperationValue,
    }

    #[derive(Debug, PartialEq)]
    enum OperationValue {
        Old,
        N(usize),
    }

    impl TryFrom<&str> for OperationValue {
        type Error = &'static str;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            match s {
                "old" => Ok(OperationValue::Old),
                n => Ok(OperationValue::N(
                    n.parse().map_err(|_| "invalid operation value")?,
                )),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum Operator {
        Add,
        Multiply,
    }

    impl TryFrom<&str> for Operator {
        type Error = &'static str;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            match s {
                "*" => Ok(Operator::Multiply),
                "+" => Ok(Operator::Add),
                _ => Err("invalid operator"),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_try_from_str() {
            let input = include_str!("test.txt").split("\n\n").next().unwrap();
            let expected = Monkey {
                items: vec![Item { worry_level: 79 }, Item { worry_level: 98 }],
                operation: Operation {
                    a: OperationValue::Old,
                    op: Operator::Multiply,
                    b: OperationValue::N(19),
                },
                test: 23,
                action_true: 2,
                action_false: 3,
            };

            assert_eq!(Monkey::try_from(input), Ok(expected));
        }
    }
}

fn main() {
    println!("Hello, world!");
}
