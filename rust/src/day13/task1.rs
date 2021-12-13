use crate::utils::load_input;
use std::collections::HashSet;

#[derive(Debug)]
enum Coord {
    X(usize),
    Y(usize),
}

pub fn solve() -> usize {
    let input = load_input(13).join("\n");
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots: Vec<(usize, usize)> = dots
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

    let fold = &folds[0];
    let dots: HashSet<(usize, usize)> = dots
        .into_iter()
        .map(|(x, y)| {
            if let Coord::X(fold_x) = fold {
                if x > *fold_x {
                    return (2 * fold_x - x, y);
                }
            }
            (x, y)
        })
        .collect();
    dots.len()
}
