use std::io;

fn main() {
    let mut age_input = String::new();
    println!("Please enter your age:");
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read line");

    let age: u32 = age_input.trim().parse().expect("please type a number");

    println!("Your age: {}", age);

    let age_in_days = age * 365;
    println!("Age in Days {}", age_in_days);
}
