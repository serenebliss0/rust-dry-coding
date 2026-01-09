use semire_read::Readable;

pub fn define_days(day: u8) -> &'static str 
{
    match day {
        1 => "a partridge in a pear tree",
        2 => "two turtle doves",
        3 => "three French hens",
        4 => "four calling birds",
        5 => "five golden rings", //gonna be gonna be, gonna be gonna be goldennnnnnnnn
        6 => "six geese a-laying",
        7 => "seven swans a-swimming",
        8 => "eight maids a-milking",
        9 => "nine ladies dancing",
        10 => "ten lords a-leaping",
        11 => "eleven pipers piping",
        12 => "twelve drummers drumming",
        _ => ""
    }
}

pub fn ordinal(day: u8) -> &'static str 
{
    match day 
    {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => ""
    }
}

pub fn respective_day(day:u8) 
{
        println!("On the {} day of Christmas", ordinal(day));
        println!("My true love gave to me");

        for gift in (1..=day).rev() 
        {
            if gift == 1 && day != 1 
            {
                println!("And {}", define_days(gift));
            } 
            else 
            {
                println!("{}", define_days(gift));
            }
        }

        println!(); // blank line between verses
    }

fn main() {
    println!("What day of christmas is it?");
    let day = u8::read();

    if day > 12
    {
        println!("Christmas is over buddy");
    }
    else
    {
        respective_day(day);
    }
}