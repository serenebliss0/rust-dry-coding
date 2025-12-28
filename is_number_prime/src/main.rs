use std::io;

fn main()
{
    println!("Enter a positive integer!");

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number:i64 = match number.trim().parse() {
        Ok(number) => number,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if number % 2 != 0 && number % 3 != 0 && number % 5 != 0 && number % 7 != 0
    {
        if number == 1
        {
            println!("This is not a prime number");
        }
        else
        {
            println!("This is a prime number!");
        }
    }
    else if number == 2 || number == 3 || number == 5 || number == 7
        {
            println!("This is a prime number!");
        }
    else
    {
        println!("This is not a prime number");
    }
}
