use crate::day11::task1::{get_monkey_business, Monkey};

pub fn solution(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
    let normalizer = monkeys.iter().map(|m| m.test).product();
    for _round in 1..=10000 {
        for monkey_index in 0..monkeys.len() {
            while !monkeys[monkey_index].items.is_empty() {
                let old_item = monkeys[monkey_index].items.pop_front().unwrap();
                let item = monkeys[monkey_index].inspect(old_item, Some(normalizer));
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
            2713310158
        );
    }
}
