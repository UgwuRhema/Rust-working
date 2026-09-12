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
    let mut students: Vec<Student> = Vec::new();
    let student1: Student = Student::new("Ugwu Rhema", 17, 1);
    let student2: Student = Student::new("Phat Nickher", 16, 2);
    let student3: Student = Student::new("Balls Kennedy", 17, 3);
    students.push(student1);
    students.push(student2);
    students.push(student3);

    for i in 0..3 {
        students[i].print_stud();
    }
}
