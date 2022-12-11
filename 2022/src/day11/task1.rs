use itertools::Itertools;
use std::collections::VecDeque;

pub struct Monkey {
    pub items: VecDeque<usize>,
    operator: String,
    right: String,
    pub test: usize,
    pub if_true: usize,
    pub if_false: usize,
    inspect_count: usize,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        fn usize_from_last_word(s: &str) -> usize {
            s.split(' ')
                .last()
                .unwrap()
                .parse()
                .expect("Could not parse usize")
        }
        let (items_raw, inspect_raw, test_raw, if_true_raw, if_false_raw) =
            input.trim().lines().skip(1).collect_tuple().unwrap();
        let items = items_raw
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let (_, operator, right) = inspect_raw
            .split("= ")
            .nth(1)
            .unwrap()
            .split(' ')
            .collect_tuple()
            .unwrap();

        Self {
            items,
            right: right.to_string(),
            operator: operator.to_string(),
            test: usize_from_last_word(test_raw),
            if_true: usize_from_last_word(if_true_raw),
            if_false: usize_from_last_word(if_false_raw),
            inspect_count: 0,
        }
    }
}

impl Monkey {
    pub fn inspect(&mut self, x: usize, normalizer: Option<usize>) -> usize {
        self.inspect_count += 1;
        let base = match self.operator.as_str() {
            "+" => x + self.right.parse().unwrap_or(x),
            "*" => x * self.right.parse().unwrap_or(x),
            _ => panic!("Unknown operation"),
        };
        if let Some(normalizer) = normalizer {
            base % normalizer
        } else {
            base
        }
    }
}

pub fn solution(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            while !monkeys[monkey_index].items.is_empty() {
                let old_item = monkeys[monkey_index].items.pop_front().unwrap();
                let item = monkeys[monkey_index].inspect(old_item, None) / 3;
                if item % monkeys[monkey_index].test == 0 {
                    let if_true = monkeys[monkey_index].if_true;
                    monkeys[if_true].items.push_back(item);
                } else {
                    let if_false = monkeys[monkey_index].if_false;
                    monkeys[if_false].items.push_back(item);
                }
            }
        }
    }
    get_monkey_business(&monkeys)
}

pub fn get_monkey_business(monkeys: &[Monkey]) -> usize {
    monkeys
        .iter()
        .map(|monkey| monkey.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    "
            ),
            10605
        );
    }
}
