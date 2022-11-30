pub mod task1;
pub mod task2;

pub fn run() {
    println!("{}", task1::count_valid_passports().unwrap());
    println!("{}", task2::count_valid_passports().unwrap());
}
