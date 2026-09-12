use std::io::{self, Write};

struct Student
{
    name: String,
    age: u32,
    id: u32,
}

impl Student
{
    fn new(n: &str, a: u32, i: u32) -> Self
    {
        Self {
            name: n.to_string(),
            age: a,
            id: i
        }
    }

    fn print_stud(&self)
    {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Id: {}", self.id);
    }
}

fn main()
{
    let Students: Vec<Student> = Vec::new();
}
