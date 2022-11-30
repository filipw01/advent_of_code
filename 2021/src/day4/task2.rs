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
        while let Some(board) = filter_boards(&mut boards, *input_value) {
            if boards.len() == 0 {
                return board
                    .fields
                    .into_iter()
                    .filter(|field| !field.marked)
                    .map(|field| field.value)
                    .sum::<usize>()
                    * input_value;
            }
        }
    }
    return 0;
}

fn filter_boards(boards: &mut Vec<Board>, input: usize) -> Option<Board> {
    for (board_index, board) in boards.into_iter().enumerate() {
        let fields: &mut Vec<Field> = board.fields.borrow_mut();
        if let Some(mut field) = fields.into_iter().find(|field| field.value == input) {
            field.marked = true;
            for i in 0..5 {
                if (fields
                    .into_iter()
                    .filter(|field| field.x == i)
                    .all(|field| field.marked))
                    || (fields
                        .into_iter()
                        .filter(|field| field.y == i)
                        .all(|field| field.marked))
                {
                    return Some(boards.remove(board_index));
                }
            }
        }
    }
    None
}
