use crate::utils::load_input;

pub mod task1;
pub mod task2;
mod visualization;

pub fn run() {
    let input = load_input(1);
    println!("{}", task1::solution(&input));
    println!("{}", task2::solution(&input));
    visualization::main();
}
