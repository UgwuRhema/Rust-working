use std::io::{self, Write};
use std::fs::File;

fn main() -> std::io::Result<()>
{
    let filename: String = String::from("file.txt");
    let mut file = File::create(filename)?;
    file.write_all(b"Hello World from file.txt")?;
    Ok(())
}
