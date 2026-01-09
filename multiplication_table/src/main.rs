use semire_read::Readable;

pub fn do_multiplication(number:&i64, max_multiple:u64)
{
    for multiple in 0..=max_multiple
    {
        println!("{} x {} = {}", number, multiple, (number*multiple as i64));
    }
}

fn main()
{
    println!("Enter a number!");
    let number = i64::read();

    println!("How many multiples of {} do you need?", number);
    let max_multiple = u64::read();

    do_multiplication(&number, max_multiple);
}