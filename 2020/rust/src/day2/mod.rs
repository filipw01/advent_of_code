pub mod task1;
pub mod task2;

pub fn run() {
    println!("{}", task1::check_password().unwrap());
    println!("{}", task2::check_password().unwrap());
}
