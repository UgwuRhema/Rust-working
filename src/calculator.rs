use std::io::{self, Write}; //don't forget the libraries...

fn main()
{
    //let me create variables here first...
    let mut num_a: String = String::new();
    let mut num_b: String = String::new(); //since stdio only reads into owned Strings...
    //the acytual variables...
    let num1: u64;
    let num2: u64;
    println!("Welcome to Calculator!");

    print!("Enter first number: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin().read_line(&mut num_a).expect("Failed to Read into num_a");
    num1 = num_a.trim().parse().expect("Please type a valid number!");

    //woah what is even this, just to read integers, man i dont like this...
    print!("Enter second number: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin().read_line(&mut num_b).expect("Failed to Read into num_b");
    num2 = num_b.trim().parse().expect("Please type a valid number!");

    
}
