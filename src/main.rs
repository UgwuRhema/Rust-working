struct Human {
	name: String,
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn main(){
    let a: u64 = 56;
    let b: u64 = 50;
    let result: u64 = add(a,b);
    println!("The result is {}", result);
}
