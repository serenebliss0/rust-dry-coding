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

pub fn do_armstrong_with_range(lower_bound:u32, upper_bound:u32)
{
    for i in lower_bound..upper_bound
    {
        match check_armstrong(i.to_string())
    {
        true => {
            println!("{} is a narcissistic number!", i);
        },
        false => {
            
        }
    }
}
}
fn main()
{
    println!("Enter a range and find all the narcissistic between them!");

    println!("Enter a lower bound!");
    let lower_bound = u32::read();

    println!("Enter an upper bound!");
    let upper_bound = u32::read();

    do_armstrong_with_range(lower_bound, upper_bound);
}
