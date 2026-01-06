use semire_read::Readable;

pub fn count_words(sentence:String) -> u64
{
    sentence.split_whitespace().count() as u64
}

fn main()
{
    println!("Konnichiwa!");
    println!("Enter multiple words below:");

    let sentence = String::read();

    println!("You have {} words in what you typed!!", count_words(sentence));
}
