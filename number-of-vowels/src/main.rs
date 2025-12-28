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

pub fn count_vowel(word:String) -> u32
{
    
    let mut char_array: Vec<char> = word.chars().collect();

    let mut total_vowel_count = 0;
    let mut a = 0;
    let mut e = 0;
    let mut i = 0;
    let mut o = 0;
    let mut u = 0;

    for letter in 0..char_array.len()
    {
        if char_array[letter] == 'a'
        {
            a+=1;
        }

        if char_array[letter] == 'e'
        {
            e+=1;
        }

        if char_array[letter] == 'i'
        {
            i+=1;
        }
        if char_array[letter] == 'o'
        {
            o+=1;
        }
        if char_array[letter] == 'u'
        {
            u+=1;
        }

        total_vowel_count = a + e + i + o + u;
    }
    return total_vowel_count as u32

}


fn main()
{
    println!("Enter a word");
    let mut word = read_line();

    let vowel_count = count_vowel(word);
    println!("You have {} vowels here.", vowel_count);

}