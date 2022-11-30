use std::env::args;

mod day1;
mod utils;

fn main(){
    let day = args().nth(1).expect("Provide day number as an argument");
    match day.as_str() {
        "1" => day1::run(),
        _ => panic!("Wrong day, got {}", day),
    }
    println!("Hello, world!");
}
