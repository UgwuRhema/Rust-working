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
    //just checking out somethings first...
    let mut Students: Vec<Student> = Vec::new();
    let student1: Student = Student::new("Ugwu Rhema", 17, 1);
    let student2: Student = Student::new("Phat Nickher", 16, 2);
    let student3: Student = Student::new("Balls Kennedy", 17, 3);
    Students.push(student1);
    Students.push(student2);
    Students.push(student3);
}
