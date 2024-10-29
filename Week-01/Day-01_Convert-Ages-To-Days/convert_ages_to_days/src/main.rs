use std::io;

fn main() {
    let mut age_input = String::new();
    println!("Please enter your age:");
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read line");

    let age:u32 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Please enter a valid number");
    return;}

    };
    

    println!("Your age: {}", age);

    let age_in_days = age * 365;
    println!("Age in Days {}", age_in_days);
}
