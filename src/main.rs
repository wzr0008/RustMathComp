use crate::addition::primary_set::*;

mod addition;

fn main() {
    println!("Test Add up");
    let a = 3;
    let b = 5;
    println!("a+b={}", add_up(a, b));
    let a = 3.5;
    let b = 1.2;
    println!("a+b={}", add_up(a, b));
}
