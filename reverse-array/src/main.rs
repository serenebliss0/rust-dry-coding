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
    println!("How many words would you like to enter?");

    let word_counter:u8 = match read_line().parse() {
        Ok(word_counter) => word_counter,
        Err(lagging) => {
            println!("{}", lagging);
            return;
        }
    };

    let mut  words = Vec::new();

    for i in 1..=word_counter
    {
        println!("Enter a word, letter or character for position {}", i);

        let word = read_line();

        words.push(word);
    }

    println!("You typed in {:?}", words);
    
    println!("That reversed is: ");
    for word in words.iter().rev()
    {
        print!("{}\t",word);
    }
}