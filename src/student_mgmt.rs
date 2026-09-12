use std::io::{self, Write};

struct Student
{
    name: String,
    age: u32,
    id: u32,
}

impl Student
{
    fn new(n: String, a: u32, i: u32) -> Self
    {
        Self {
            name: n,
            age: a,
            id: i
        }
    }

    fn print_stud(&self)
    {
        println!("Name: {}", self.name.trim());
        println!("Age: {}", self.age);
        println!("Id: {}", self.id);
    }
}

fn main()
{
    //student vector...
    let mut students: Vec<Student> = Vec::new();
    print!("Enter Student name: ");
    let mut name: = String = String::new();
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut name).expect("Failed to read into name");
    
}
