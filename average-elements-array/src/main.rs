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
    println!("How many numbers do you wanna add?");

    let number_count:u64 = match read_line().parse() {
        Ok(number_count) => number_count,
        Err(lag) => {
            println!("{}", lag);
            return;
        }
    };
    
    let mut numbers = Vec::new();
    
    for i in 1..=number_count
    {
        println!("Enter the next number!");

        let number:i64 = match read_line().parse() {
            Ok(number) => number,
            Err(lagging) => {
                println!("{}", lagging);
                return;
            }
        };

        numbers.push(number);
    }

    let mut sum = 0;
    for j in 0..numbers.len()
    {
        sum += numbers[j];
    }

    let len_array = numbers.len() as i64;
    let mut average = sum / len_array;

    println!("The sum of all the elements is {}", sum);
    println!("The average of all the elements is {}", average);

}