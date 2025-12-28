pub fn read_line() -> String
{
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(lag) => {
            println!("Oops: {}", lag);
            return String::new()
        }
    }
}

pub fn replace_spaces(word:String) -> String
{
    let changed_string:String = word.replace(' ', "_");
    return changed_string
}

//alternatively, this will work on spaces, tabs, and even newlines
pub fn replace_all_whitespace(word:String) -> String
{
    let changed_string = word
    .chars()
    .map(|c| if c.is_whitespace() { '_' } else { c })
    .collect::<String>();
}

fn main()
{
    println!("Enter a sentence or phrase!");
    let sentence = read_line();

    println!("Your sentence in snake cases is {}", replace_spaces(sentence));
}