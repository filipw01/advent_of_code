use crate::utils::load_input;
use std::borrow::BorrowMut;

#[derive(Debug)]
struct Field {
    x: usize,
    y: usize,
    value: usize,
    marked: bool,
}

#[derive(Debug)]
struct Board {
    fields: Vec<Field>,
}

pub fn solve() -> usize {
    let lines = load_input(4).join("\n");
    let (input, boards) = lines.split_once("\n\n").unwrap();
    let input: Vec<usize> = input
        .split(',')
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = boards
        .split("\n\n")
        .into_iter()
        .map(|board| Board {
            fields: board
                .split_whitespace()
                .into_iter()
                .enumerate()
                .map(|(index, field)| Field {
                    x: index % 5,
                    y: index / 5,
                    value: field.parse().unwrap(),
                    marked: false,
                })
                .collect(),
        })
        .collect();

    for input_value in &input {
        for board in &mut boards {
            let fields: &mut Vec<Field> = board.fields.borrow_mut();
            if let Some(mut field) = fields.into_iter().find(|field| field.value == *input_value) {
                field.marked = true;
                for i in 0..5 {
                    if fields
                        .into_iter()
                        .filter(|field| field.x == i)
                        .all(|field| field.marked)
                        || fields
                            .into_iter()
                            .filter(|field| field.y == i)
                            .all(|field| field.marked)
                    {
                        return fields
                            .into_iter()
                            .filter(|field| !field.marked)
                            .map(|field| field.value)
                            .sum::<usize>()
                            * input_value;
                    }
                }
            }
        }
    }
    0
}
