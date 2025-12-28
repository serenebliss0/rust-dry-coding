use std::io;

pub fn read_line() -> String
{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => {
            println!("Oops: {}", e);
            return String::new()
        }
    }
}

pub fn count_consonant(word:String) -> u32
{
    
    let mut char_array: Vec<char> = word.chars().collect();

    let mut total_consonant_count = 0;

    for letter in 0..char_array.len()
    {
        if char_array[letter].is_alphabetic() && char_array[letter] != 'a' && char_array[letter] != 'e' && char_array[letter] != 'i' && char_array[letter] != 'o' && char_array[letter] != 'u'
        {
            total_consonant_count+=1;
        }
    }

    return total_consonant_count as u32

}


fn main()
{
    println!("Enter a word");
    let mut word = read_line();

    let consonant_count = count_consonant(word);
    println!("You have {} consonants here.", consonant_count);

}