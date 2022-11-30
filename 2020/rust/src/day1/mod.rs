pub mod task1;
pub mod task2;

pub fn run() {
    println!("{}", task1::calculate_expense_report().unwrap());
    println!("{}", task2::calculate_expense_report().unwrap());
}
