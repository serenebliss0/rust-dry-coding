use semire_read::Readable;

pub fn find_even(array:Vec<i64>) -> Vec<i64>
{
    let mut just_even = Vec::new();

    for i in 0..array.len() -1
    {
    if array[i] % 2 == 0
    {
        just_even.push(array[i]);
    }
    }
    
    just_even.sort();

    just_even
}

pub fn largest_number(array:Vec<i64>) -> i64
{
    //(find_even(array).len()) as i64
    let length = (array.len() -1) as usize;
    let largest = array[length];
    largest
}


fn main()
{
    println!("How many numbers do you wanna enter?");
    let num_count = u8::read();

    let mut numbers = Vec::new();

    for i in 1..=num_count
    {
        println!("Enter value for position {}", i);
        numbers.push(i64::read());
    }

    numbers.sort();

    println!("The largest even number is {}",largest_number(numbers));

}