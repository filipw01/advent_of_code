use crate::utils::load_input;

pub mod task1;
pub mod task2;

pub fn run() {
    let input = load_input(15);
    println!("{}", task1::solution(&input, 2000000));
    println!("{}", task2::solution(&input, 4000000));
}
