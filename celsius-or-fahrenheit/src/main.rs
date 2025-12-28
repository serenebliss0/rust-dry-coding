pub fn read_line() -> String
{
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(lag) => {
            println!("Oops: {}", lag);
            return String::new()
        }
    }
}

pub fn read_as_number() -> f64
{
    let input_as_number:f64 = match read_line().parse() {
        Ok(input_as_number) => return input_as_number,
        Err(still_lag) => {
            println!("Oops: {}", still_lag);
            return 0.0
        }
    };

}

pub fn celsius_to_fahrenheit(temperature:f64) -> f64
{
    // f = 1.8deg c + 32
    let degrees_fahrenheit = (1.8 * temperature) + 32.00;

    degrees_fahrenheit
}

pub fn fahrenheit_to_celsius(temperature:f64) -> f64
{
    //if f = 1.8deg c + 32
    // c = (f-32)/1.8
    let degrees_celsius = (temperature - 32.00) / 1.8;

    degrees_celsius
}

fn main()
{
    println!("Welcome user! Is your temperature in celsius or fahrenheit");
    println!("Type c for celsius, f for fahrenheit");

    let temperature_unit = read_line();

    match temperature_unit.to_lowercase().as_str()
    {
        "f" => {
            println!("Enter the temperature in degrees fahrenheit.");
            let temperature = read_as_number();
            let degrees_celsius = fahrenheit_to_celsius(temperature);
            println!("Temperature in celsius is {:.2}", degrees_celsius);
        },
        "c" => {
            println!("Enter the temperature in degrees celsius.");
            let temperature = read_as_number();
            let degrees_fahrenheit = celsius_to_fahrenheit(temperature);
            println!("Temperature in fahrenheit is {:.2}", degrees_fahrenheit);
        }
        _ => {
            println!("Invalid input entered!");
        }
    }
}