use crate::utils::load_input;

pub fn solve() -> usize {
    let input = load_input(17);
    let (safe_x, safe_y) = input.get(0).unwrap().split_once(", y=").unwrap();
    let (_, safe_x) = safe_x.split_once("x=").unwrap();
    let (safe_max_y, safe_min_y) = safe_y.split_once("..").unwrap();
    let safe_max_y: isize = safe_max_y.parse().unwrap();
    let safe_min_y: isize = safe_min_y.parse().unwrap();
    let (safe_min_x, safe_max_x) = safe_x.split_once("..").unwrap();
    let safe_min_x: isize = safe_min_x.parse().unwrap();
    let safe_max_x: isize = safe_max_x.parse().unwrap();

    let mut max_possible_x = safe_max_x;
    let mut max_possible_y = -safe_max_y - 1;

    while simulate(
        (max_possible_x, max_possible_y),
        (safe_min_x, safe_max_x),
        (safe_min_y, safe_max_y),
    ) == false
    {
        max_possible_x -= 1;
        if max_possible_x == 0 {
            max_possible_y -= 1;
            max_possible_x = safe_max_x;
        }
    }

    ((max_possible_y + 1) / 2 * max_possible_y) as usize
}

fn simulate(initial: (isize, isize), safe_x: (isize, isize), safe_y: (isize, isize)) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut speed = initial;
    while x <= safe_x.1 && y >= safe_y.1 {
        x += speed.0;
        y += speed.1;
        if x >= safe_x.0 && x <= safe_x.1 && y <= safe_y.0 && y >= safe_y.1 {
            return true;
        }
        let new_x_speed = if speed.0 == 0 { 0 } else { speed.0 - 1 };
        speed = (new_x_speed, speed.1 - 1);
    }
    false
}
