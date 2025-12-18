use std::io;

fn main()
{
    let mut number = 0;

    for i in 1..100
    {
        number +=1 ;
    if number % 2 != 0 && number % 3 != 0 && number % 5 != 0 && number % 7 != 0
    {
        println!("{}", number);
    }
    else if number == 2 || number == 3 || number == 5 || number == 7
        {
            println!("{}", number);
        }
    else
    {
        continue;
    }
}
}
