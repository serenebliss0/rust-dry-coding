
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

pub fn reverse_string(word:String) -> String
{
    let mut word_array = word.chars().rev().collect::<String>();
    
    word_array
}

fn main()
{
    println!("Enter a word, or anything really");

    let mut word = read_line();
    let reversed_string = reverse_string(word);
    
    println!("The string reversed is {}", reversed_string);
}