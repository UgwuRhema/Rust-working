use std::io::{self, Write};
use std::fs::File;

fn main() -> std::io::Result<()>
{
    let filename = "file.txt";
    println!("File: {}", filename);
    Ok(())
}
