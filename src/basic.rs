use std::io::{self, Write}; //like stdio.h, for input and (i'm not sure)output...

struct Human
{
    name: String,
    age: u32,
}

//i can't lie this 'impl' block is good, but i still prefer C++ classes over this no offense
impl Human
{
    fn new(name: &str, age: u32) -> Self{
        Self {name: name.to_string(), age}
    }
}

fn main() {
    //two types of strings, string slice...(just like const char *)
    let string1: &str = "Nigger!";
    println!("{}", string1);
    //and Owned String...
    let mut string2: String = String::from("Nigger!");
    println!("{}", string2);

    //an array of unsigned 64 bit integers
    let arr: [u64; 5] = [34, 45, 12, 21, 90];

    //this acts like a pointer in a way, if you get what I'm saying
    let slice: &[u64] = &arr[1..3]; //get's 64 bit integers from index 1 - 3 and stores them into
                                    //an array slice

    println!("First element in array: {}", arr[0]);
    println!("The whole array is {:?}", arr);

    //so let's say i want to read a name...
    let mut name: String = String::new(); //note, io::stdin can only read into String variables...
    print!("Enter your name: ");

    //equivalent to fflush(stdout); it seems Rust has the same method
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Hello, {}!", name.trim());

    //we are gonna test structs and their impl blocks, let's see how they differ from C...

    //in my opinion this is better than C++ vectors but not C's mallocs and reallocs
    let mut people: Vec<Human> = Vec::new();
    people.push(Human::new("Rhema", 17));
    people.push(Human::new("Boss", 17));

    for i in 0..=2 {
        println!("Name: {}, Age: {}", people[i].name, people[i].age);
    }
}
