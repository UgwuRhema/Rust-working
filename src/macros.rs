macro_rules! give_six
{
    () => {
        6
    };
}

fn main()
{
    let mut a_value: u64 = give_six!();
    println!("The value: {}", a_value);
}
