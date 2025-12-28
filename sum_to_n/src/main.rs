use std::io;

fn main()
{
    println!("Enter a positive integer");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number:i64 = match number.trim().parse() {
        Ok(number) => number,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut counter = 0;

    for i in 0..=number
    {
        counter += i;
    }

    println!("The sum to n is {}", counter);
    }



