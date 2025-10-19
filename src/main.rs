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
    println!("Test the min stack");
    let mut stack = MinStack::new();
    stack.push(-1);
    stack.push(3);
    println!("The min value of stack is {:?}", stack.min_value());
    println!("The last value of stack is {:?}", stack.top());
    stack.pop();
    stack.pop();
    stack.push(-4);
    stack.push(-5);
    stack.push(100);
    println!("The min value of stack is {:?}", stack.min_value());
    println!("The last value of stack is {:?}", stack.top());
    println!("Test the unique letters in the string");
    println!("Input the string is \"abaccdeff\" ");
    println!("The output of unique letter is {:?}",not_repeat_char("abaccdeff"));
}
