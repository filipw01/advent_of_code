use crate::utils::load_input;

pub fn solve() -> usize {
    let lines = load_input(6);
    let mut school: Vec<usize> = lines
        .get(0)
        .unwrap()
        .split(',')
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect();

    for _ in 0..80 {
        let mut next_gen = vec![];
        for fish in school {
            if fish == 0 {
                next_gen.push(8);
                next_gen.push(6);
            } else {
                next_gen.push(fish - 1)
            }
        }
        school = next_gen;
    }
    school.len()
}
