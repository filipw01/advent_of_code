use crate::utils::load_input;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
enum Coord {
    X(usize),
    Y(usize),
}

pub fn solve() -> usize {
    let input = load_input(13).join("\n");
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots: HashSet<(usize, usize)> = dots
        .split('\n')
        .map(|dot| dot.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds: Vec<Coord> = folds
        .split('\n')
        .map(|fold| fold.split_once('=').unwrap())
        .map(|(text, coord)| match text.chars().last().unwrap() {
            'x' => Coord::X(coord.parse().unwrap()),
            'y' => Coord::Y(coord.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect();

    let final_dots = folds
        .into_iter()
        .fold(dots, |dots, fold| fold_paper(dots, &fold));
    println!("{:?}", final_dots);

    let rendered = itertools::repeat_n(0, 6)
        .enumerate()
        .map(|(y, _)| {
            itertools::repeat_n(0, 39)
                .enumerate()
                .map(|(x, _)| {
                    if final_dots.contains(&(x, y)) {
                        'x'
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .join("\n");
    println!("{}", rendered);
    0
}

fn fold_paper(dots: HashSet<(usize, usize)>, fold: &Coord) -> HashSet<(usize, usize)> {
    dots.into_iter()
        .map(|(x, y)| {
            if let Coord::X(fold_x) = fold {
                if x > *fold_x {
                    return (2 * fold_x - x, y);
                }
            }
            if let Coord::Y(fold_y) = fold {
                if y > *fold_y {
                    return (x, 2 * fold_y - y);
                }
            }
            (x, y)
        })
        .collect()
}
