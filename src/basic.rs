use std::io; //like stdio.h, for input and (i'm not sure)output...

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
    let slice: &[u64] = &arr[1..3];

    println!("First element in array: {}", arr[0]);
    println!("The whole array is {:?}", arr);

    //so let's say i want to read a name...
    let mut name: String = String::new();
    print!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line").trim();

    println!("Hello, {}!", name);
}
