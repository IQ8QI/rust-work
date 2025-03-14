use std::io;

fn main() {
    let days_in_year: u16 = 365;

    println!("How old are You? (years)");

    let mut years = String::new();

    io::stdin()
        .read_line(&mut years)
        .expect("Failed to read line");

    let years: u16 = years.trim().parse().expect("Please type a number!");

    let days: u16 = days_in_year * years;

    println!("You are {} days old!", days);
}
