mod addition;

use addition::*;
use std::time::{Instant};
fn main() {
    println!("Hello, world!");
    let time_start = Instant::now();
    println!("The total of primary is {}", primary_set::addition(100000));
    let elapsed = time_start.elapsed();
    println!("Time elapsed is {:?}", elapsed);
}
