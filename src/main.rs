use std::fs;
use std::io;

mod day1;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let day: u32 = input.trim().parse().expect("Not a number");

    let filename = format!("input/{}.in", day);
    let contents = fs::read_to_string(filename).expect("unable to read");

    match day {
        1 => day1::solve(contents),
        _ => println!("Not a valid day"),
    }
}
