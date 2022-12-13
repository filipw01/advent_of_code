use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum PacketToken {
    List(Vec<PacketToken>),
    Number(u32),
}

fn trim_start_one(s: &str, pat: char) -> &str {
    if s.starts_with(pat) {
        &s[1..]
    } else {
        s
    }
}

fn trim_end_one(s: &str, pat: char) -> &str {
    if s.ends_with(pat) {
        &s[..s.len() - 1]
    } else {
        s
    }
}

fn get_items(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut depth = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ => {}
        }
        if depth == 0 && c == ',' {
            result.push(&s[start..i]);
            start = i + 1;
        }
    }
    result.push(&s[start..]);
    result
}

impl From<&str> for PacketToken {
    fn from(s: &str) -> Self {
        if s.starts_with('[') {
            let list = get_items(trim_end_one(trim_start_one(s, '['), ']'))
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| PacketToken::from(*s))
                .collect();
            PacketToken::List(list)
        } else {
            PacketToken::Number(s.parse().unwrap())
        }
    }
}

#[derive(Debug, Clone)]
struct PacketPair {
    left: Vec<PacketToken>,
    right: Vec<PacketToken>,
}

impl From<&str> for PacketPair {
    fn from(s: &str) -> Self {
        let (left, right) = s.trim().split('\n').collect_tuple().unwrap();
        let left_token = PacketToken::from(left);
        let right_token = PacketToken::from(right);
        match left_token {
            PacketToken::Number(_) => panic!("Left token is not a list"),
            PacketToken::List(left_list) => match right_token {
                PacketToken::Number(_) => panic!("Right token is not a list"),
                PacketToken::List(right_list) => PacketPair {
                    left: left_list,
                    right: right_list,
                },
            },
        }
    }
}

impl From<(PacketToken, PacketToken)> for PacketPair {
    fn from((left, right): (PacketToken, PacketToken)) -> Self {
        match left {
            PacketToken::Number(_) => panic!("Left token is not a list"),
            PacketToken::List(left_list) => match right {
                PacketToken::Number(_) => panic!("Right token is not a list"),
                PacketToken::List(right_list) => PacketPair {
                    left: left_list,
                    right: right_list,
                },
            },
        }
    }
}

impl PacketPair {
    fn is_in_right_order(&self) -> bool {
        let left_token = PacketToken::List(self.left.clone());
        let right_token = PacketToken::List(self.right.clone());
        self.compare((left_token, right_token)).unwrap_or(true)
    }

    fn compare(&self, (left, right): (PacketToken, PacketToken)) -> Option<bool> {
        match (left, right) {
            (PacketToken::Number(left_number), PacketToken::Number(right_number)) => {
                if left_number < right_number {
                    Some(true)
                } else if left_number > right_number {
                    Some(false)
                } else {
                    None
                }
            }
            (PacketToken::List(left_list), PacketToken::List(right_list)) => {
                let left_len = left_list.len();
                let right_len = right_list.len();
                let diff = left_list
                    .into_iter()
                    .zip(right_list.into_iter())
                    .find(|(left, right)| self.compare((left.clone(), right.clone())).is_some());

                if let Some(result) = diff {
                    self.compare(result)
                } else if left_len < right_len {
                    Some(true)
                } else if left_len > right_len {
                    Some(false)
                } else {
                    None
                }
            }
            (PacketToken::Number(left_number), PacketToken::List(right_list)) => self.compare((
                PacketToken::List(vec![PacketToken::Number(left_number)]),
                PacketToken::List(right_list),
            )),
            (PacketToken::List(left_list), PacketToken::Number(right_number)) => self.compare((
                PacketToken::List(left_list),
                PacketToken::List(vec![PacketToken::Number(right_number)]),
            )),
        }
    }
}

pub fn solution(input: &str) -> usize {
    let mut packets = "".to_string();
    let divider_packet = "[[2]]";
    let divider_packet_2 = "[[6]]";
    packets.push_str(input);
    packets.push_str(divider_packet);
    packets.push('\n');
    packets.push_str(divider_packet_2);

    packets
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(PacketToken::from)
        .sorted_by(|left, right| {
            let pair = PacketPair::from((left.clone(), right.clone()));
            if pair.is_in_right_order() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .enumerate()
        .filter(|(index, packet)| {
            println!("{index}, {:?}", packet);
            PacketToken::from(divider_packet) == *packet
                || PacketToken::from(divider_packet_2) == *packet
        })
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"
            ),
            140
        );
    }
}
