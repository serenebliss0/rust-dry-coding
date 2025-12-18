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
    println!("How many numbers do you wanna add to the array?");

    let number_count:usize = match read_line().parse() {
        Ok(number_count) => number_count,
        Err(eat_bread) => {
            println!("{}", eat_bread);
            return;
        }
    };

    //first lets collect the input

    let mut  numbers = Vec::new();
    for i in 1..=number_count
    {
        println!("Enter number {}", i);

        let number:i64 = match read_line().parse() {
            Ok(number) => number,
            Err(eat_more_bread) =>
            {
                println!("{}", eat_more_bread);
                return;
            }
        };

        numbers.push(number);
    }

    //now lets do the actual bubble sort

    for j in 0..number_count
    {
            for k in 0..(number_count - j  - 1)
            {
                if numbers[k] < numbers[k+1]
                {
                    numbers.swap(k, k+1);
                } 
            }
    }

    println!("The sorted array is {:?}", numbers);
    println!("The second largest element is {} ", numbers[0+1]);
}