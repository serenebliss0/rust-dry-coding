pub fn read_line() -> String
{
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(lag) => {
            println!("Oops: {}", lag);
            return String::new()
        }
    }
}

fn main()
{
    println!("Enter a sentence");
    let word = read_line();

    println!("In lowercase:\n{}", word.to_lowercase());
}