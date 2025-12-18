use std::{io, vec};

fn main() {
    println!("Welcome! How many numbers are you working with?");

    let mut number_count = String::new();
    io::stdin().read_line(&mut number_count).expect("Failed to read line");
    let number_count: u8 = match number_count.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut numbers = Vec::new();

    for i in 1..=number_count {
        println!("Enter number {}", i);
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");

        let number: i64 = match number.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        numbers.push(number);
    }

    println!("Your numbers are {:?}", numbers);

    // Sort descending
    let mut numbers_sorted = numbers.iter().max().unwrap();
    println!("The largest number you typed is {}", numbers_sorted);
}

