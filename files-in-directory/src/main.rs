use std::fs;
use semire_read::Readable;

fn main() 
{
    println!("Enter a file path to read!");
    let path = String::read();

    for entry in fs::read_dir(path).unwrap() 
    {
        let entry = entry.unwrap(); // unwrap Result
        let path = entry.path();
        println!("{}", path.display());
    }
}
