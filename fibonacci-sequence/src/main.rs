pub fn calculate_fibonacci(n:u64) -> u64
{
    if n <= 1
    {
        return 1
    }
    else
    {
        let fib = calculate_fibonacci(n-1) + calculate_fibonacci(n-2);
        return fib
    }
}

pub fn read_line() -> String
{
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) =>
        {
            println!("{}", e);
            return String::new()
        }
    }
}

fn main()
{
    println!("Welcome to the fibonacci sequence calculator");
    println!("What term of the sequence would you like to know?");

    let term_in_series:u64 = match read_line().parse() {
        Ok(term_in_series) => term_in_series,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    for i in 1..=term_in_series
    {
        println!("The {} term is {}", i, calculate_fibonacci(i));
    }
}