use semire_read::Readable;

pub fn decide_odd_number(array: &Vec<i64>) -> Vec<i64> 
{
    array
        .iter()
        .filter(|n| *n % 2 != 0)
        .cloned()
        .collect()
}

pub fn smallest_odd(mut odd_array: Vec<i64>) -> Option<i64> {
    odd_array.sort();
    odd_array.first().cloned()
}

fn main() {
    let mut numbers: Vec<i64> = Vec::new();

    loop 
    {
        println!("Enter some numbers! Press \"e\" to stop");

        let input = String::read();

        if input == "e" 
        {
            break;
        }

        match input.parse::<i64>() 
        {
            Ok(num) => numbers.push(num),
            Err(_) => 
            {
                println!("Invalid input, try again.");
                continue;
            }
        }
    }

    let odd_numbers = decide_odd_number(&numbers);

    match smallest_odd(odd_numbers) 
    {
        Some(value) => println!("Smallest odd number is {}", value),
        None => println!("No odd numbers found"),
    }
}
