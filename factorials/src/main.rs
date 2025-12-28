use std::io;

fn main()
{
    println!("Enter a number");

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let mut number:u64 = match number.trim().parse() {
        Ok(number) => number,
        Err(e) => {
            println!("Oops\n{}", e);
            return;
        }
    };

    let mut factorial = 1;

    while number > 1
    {

        factorial *= number;
        number -= 1;
    }
    
    println!("Factorial is {}", factorial);
}
    

