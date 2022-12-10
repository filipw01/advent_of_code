pub fn solution(input: &str) -> i32 {
    let mut signal_strengths = [0; 6];
    let mut cycle = 1;
    let mut x = 1;
    for line in input.lines() {
        let mut split = line.split(' ');
        let instruction = split.next().unwrap();
        match instruction {
            "addx" => {
                let value = split.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
                check_signal(&mut signal_strengths, cycle, x);
                cycle += 1;
                x += value;
                check_signal(&mut signal_strengths, cycle, x);
            }
            "noop" => {
                cycle += 1;
                check_signal(&mut signal_strengths, cycle, x);
            }
            _ => panic!("Unknown instruction"),
        }
    }
    signal_strengths.iter().sum()
}

fn check_signal(signal_strengths: &mut [i32; 6], cycle: i32, x: i32) {
    if (cycle + 20) % 40 == 0 {
        signal_strengths[(((cycle + 20) / 40) - 1) as usize] = cycle * x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
            ),
            13140
        );
    }
}
