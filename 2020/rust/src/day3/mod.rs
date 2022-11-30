pub mod task1;
pub mod task2;

pub fn run() {
    println!("{}", task1::calculate_encounters().unwrap());
    println!("{}", task2::calculate_solution().unwrap());
}
