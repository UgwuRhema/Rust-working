fn add(mut a: u64, mut b: u64) -> u64 {
    a + b
}

fn main(){
    let a: u64 = 56;
    let b: u64 = 50;
    let result: u64 = add(a,b);
    println!("The resukt is {}", result);
}
