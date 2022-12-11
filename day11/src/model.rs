use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MONKEY_RE: Regex = Regex::new(r"Monkey [0-9]+:\n.*: (?P<items>[0-9 ,]+)\n.*= (?P<a>old|[0-9]+) (?P<op>[\*+]) (?P<b>old|[0-9]+)\n.*y (?P<test>[0-9]+)\n.* (?P<action_true>[0-9]+)\n.* (?P<action_false>[0-9]+)").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct ItemThrow {
    item: Item,
    target_monkey: usize,
}

impl ItemThrow {
    pub fn get_target_monkey(&self) -> usize {
        self.target_monkey
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
pub struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    // divisible by
    test: usize,
    // monkey number if true
    action_true: usize,
    // monkey number if false
    action_false: usize,
    // how many items were inspected
    inspect_count: usize,
}

impl Monkey {
    pub fn get_test(&self) -> usize {
        self.test
    }

    pub fn get_inspect_count(&self) -> usize {
        self.inspect_count
    }

    pub fn accept_throw(&mut self, throw: &ItemThrow) {
        self.items.push(throw.item.clone());
    }

    // inspect all owned items. generate ItemThrows
    pub fn inspect_all_items<F: Fn(usize) -> usize>(&mut self, f: F) -> Vec<ItemThrow> {
        let mut throws = vec![];

        let items = self.items.clone();
        for item in &items {
            throws.push(self.inspect_item(item, &f));
        }
        self.items.clear();

        throws
    }

    fn inspect_item<F: Fn(usize) -> usize>(&mut self, item: &Item, f: F) -> ItemThrow {
        self.inspect_count += 1;

        let worry_level = self.worry_level_on_inspection(item.worry_level);
        let bored_monkey_worry_level = f(worry_level);

        let target_monkey = if bored_monkey_worry_level % self.test == 0 {
            self.action_true
        } else {
            self.action_false
        };

        let item = Item {
            worry_level: bored_monkey_worry_level,
        };

        ItemThrow {
            item,
            target_monkey,
        }
    }

    fn worry_level_on_inspection(&self, worry_level: usize) -> usize {
        let old = worry_level;
        let a = match self.operation.a {
            OperationValue::Old => old,
            OperationValue::N(n) => n,
        };
        let b = match self.operation.b {
            OperationValue::Old => old,
            OperationValue::N(n) => n,
        };
        match self.operation.op {
            Operator::Multiply => a * b,
            Operator::Add => a + b,
        }
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items: Vec<_> = self
            .items
            .iter()
            .map(|item| item.worry_level.to_string())
            .collect();
        write!(f, "Monkey ({}):  {}", self.inspect_count, items.join(", "))
    }
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
            inspect_count: 0,
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
            inspect_count: 0,
        };

        assert_eq!(Monkey::try_from(input), Ok(expected));
    }

    #[test]
    fn test_inspect_item() {
        let mut monkey =
            Monkey::try_from(include_str!("test.txt").split("\n\n").next().unwrap()).unwrap();
        let item = Item { worry_level: 79 };
        let item_after = Item { worry_level: 500 };

        // item thrown to monkey number 3
        assert_eq!(
            monkey.inspect_item(&item, |x| x / 3),
            ItemThrow {
                item: item_after,
                target_monkey: 3
            }
        );
    }

    #[test]
    fn test_worry_level_on_inspection() {
        let monkey =
            Monkey::try_from(include_str!("test.txt").split("\n\n").next().unwrap()).unwrap();

        assert_eq!(monkey.worry_level_on_inspection(79), 1501);
    }
}
