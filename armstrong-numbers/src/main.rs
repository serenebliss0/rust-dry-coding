use semire_read::Readable;

pub fn check_armstrong(number:String) -> bool
{
    let num_array: Vec<i32> = number
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    
    let len_array = num_array.len();
    let mut sum:i32 = 0;

    for i in 0..num_array.len()
    {
        sum += (num_array[i]).pow(len_array as u32);
    }

    if sum == vec_to_number(num_array)
    {
        true
    }
    else
    {
        false
    }
    
}

fn vec_to_number(v: Vec<i32>) -> i32 {
    v.iter()
    .map(|x| x.to_string())
    .collect::<String>()
    .parse()
    .unwrap()
}


fn main()
{
    println!("Enter a number and see if it's narcissistic or not!");

    let number = String::read();

    match check_armstrong(number)
    {
        true => {
            println!("This is a narcissistic number!");
        },
        false => {
            println!("This isn't a narcissistic number");
        }
    }
}