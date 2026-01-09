use semire_read::Readable;

pub fn print_vertically(value:String)
{
    let char_array:Vec<char> = value.chars().collect();
    
    for i in 0..char_array.len()
    {
        println!("{}", char_array[i]);
    }
}

fn main()
{
    println!("Enter a number!");
    let value = String::read();

    print_vertically(value);
}