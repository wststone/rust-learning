fn main() {
    println!("Hello, world!");
    let x: u64 = 42;
    println!("The meaning of life is {}", x);
    if meaning_of_life(x) {println!("No Life")} else {println!("Yes Life")}
}

fn meaning_of_life(num: u64) -> bool {
    if num != 42 {return true}
    return false
}
