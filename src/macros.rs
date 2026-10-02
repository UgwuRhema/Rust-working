//simple macro from a page...
macro_rules! give_six
{
    () => {
        6
    };
}

/* it seems macros can perform more than one tasks */
macro_rules! new_macro
{
    ($name:expr) => {
        $name
    };

    ($text:expr) => {
        print!("{}", $text);
    };
}

fn main()
{
    let mut a_value: u64 = give_six!();
    println!("The value: {}", a_value);

    /* let's try to use new_macro */
    let name = new_macro!(String::from("Nigger"));
    print!("{}", name);
}
