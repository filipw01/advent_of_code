use crate::utils::load_input;

#[derive(Debug)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

pub fn solve() -> usize {
    let lines = load_input(11);
    let mut octopuses: Vec<Vec<Octopus>> = lines
        .iter()
        .map(|octopus_line| {
            octopus_line
                .chars()
                .map(|c| Octopus {
                    energy: c.to_digit(10).unwrap() as u8,
                    flashed: false,
                })
                .collect()
        })
        .collect();

    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += simulate(&mut octopuses);
    }
    total_flashes
}

fn simulate(octopuses: &mut Vec<Vec<Octopus>>) -> usize {
    octopuses.iter_mut().for_each(|octopus_line| {
        octopus_line.iter_mut().for_each(|octopus| {
            octopus.energy += 1;
            octopus.flashed = false;
        })
    });
    let mut flashes = 0;
    for y in 0..octopuses.len() {
        for x in 0..octopuses.len() {
            flashes += flash(octopuses, (x, y));
        }
    }
    flashes
}

fn flash(octopuses: &mut Vec<Vec<Octopus>>, (x, y): (usize, usize)) -> usize {
    let octopus: &mut Octopus = octopuses.get_mut(y).unwrap().get_mut(x).unwrap();
    if octopus.energy < 10 {
        return 0;
    }
    let mut flashes = 1;
    octopus.flashed = true;
    octopus.energy = 0;
    for y_diff in 0..3 {
        for x_diff in 0..3 {
            let new_y = y as isize + y_diff - 1;
            let new_x = x as isize + x_diff - 1;
            if new_y < 0 || new_x < 0 {
                continue;
            }
            let mut empty_vec: Vec<Octopus> = vec![];
            let mut empty_octopus = Octopus {
                energy: 0,
                flashed: true,
            };
            let octopus: &mut Octopus = octopuses
                .get_mut(new_y as usize)
                .unwrap_or(&mut empty_vec)
                .get_mut(new_x as usize)
                .unwrap_or(&mut empty_octopus);
            if octopus.flashed == true {
                continue;
            }

            octopus.energy += 1;
            if octopus.energy > 9 {
                flashes += flash(octopuses, (new_x as usize, new_y as usize));
            }
        }
    }
    flashes
}
