use std::io;

pub fn read_line() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();

    input
}

fn main()
{
    println!("How many numbers are you working with?");

    let number_count:u8 = match read_line().parse() {
        Ok(number_count) => number_count,
        Err(lag) => {
            println!("{}", lag);
            return;
        }
    };

    let mut numbers = Vec::new();

    for i in 1..=number_count
    {
        println!("Enter number {}", i);

        let number:i64 = match read_line().parse() {
            Ok(number) => number,
            Err(lagging) => {
                println!("{}", lagging);
                return;
            }
        };

        numbers.push(number);
    }

    println!("You typed in {:?}", numbers);
    
    let smallest = numbers.iter().min().unwrap();
    println!("The smallest value is {}", smallest);
}