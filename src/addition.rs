#[allow(dead_code)]
pub mod primary_set {
    use std::cmp::min;
    use std::collections::HashMap;
    use std::ops::Add;

    pub fn addition(upperbound: u32) -> u128 {
        let mut total = 0;
        for val in 2..=upperbound {
            let mut check = true;
            for part in 2..val {
                if val % part == 0 {
                    check = false;
                    break;
                }
            }
            if check {
                total += val as u128;
            }
        }
        return total;
    }
    pub fn add_up<T>(a: T, b: T) -> T
    where
        T: Add<Output = T>,
    {
        a + b
    }

    pub struct MinStack<T>
    where
        T: Ord + PartialOrd,
    {
        stack: Vec<T>,
        stack_min: Vec<T>,
    }
    impl<T> MinStack<T>
    where
        T: Ord + PartialOrd + Copy,
    {
        pub fn new() -> Self {
            let stack = Vec::new();
            let stack_min = Vec::new();
            return Self { stack, stack_min };
        }
        pub fn push(&mut self, val: T) {
            if self.stack_min.len() > 0 {
                let mut min_val = *self.stack_min.iter().last().unwrap();
                min_val = min(min_val, val);
                self.stack_min.push(min_val);
            } else {
                self.stack_min.push(val);
            }
            self.stack.push(val);
        }
        pub fn pop(&mut self) {
            self.stack_min.pop();
            self.stack.pop();
        }
        pub fn top(&self) -> Result<&T, String> {
            if self.stack_min.len() == 0 {
                return Err("The stack is empty".to_string());
            }
            return Ok(self.stack.iter().last().unwrap());
        }
        pub fn min_value(&self) -> Result<&T, String> {
            if self.stack_min.len() == 0 {
                return Err("The stack is empty".to_string());
            }
            return Ok(self.stack_min.iter().last().unwrap());
        }
    }
    pub fn not_repeat_char(s: &str) -> Result<Vec<char>, String> {
        let res = s
            .chars()
            .fold(HashMap::new(), |mut acc, x| {
                let mut entry = acc.entry(x).or_insert(0);
                *entry += 1;
                return acc;
            })
            .iter()
            .filter(|acc| acc.1 == &1)
            .map(|acc| acc.0.clone())
            .collect::<Vec<char>>();
        if res.len() == 0 {
            return Err(String::from("The stack is empty"));
        }
        return Ok(res);
    }
    pub fn replace_space(str: &str) -> Result<String, String> {
        let res = str
            .chars()
            .into_iter()
            .map(|x| {
                if x == ' ' {
                    return "%20".to_string();
                } else {
                    return x.to_string();
                }
            })
            .collect::<String>();
        if res.len() == 0 {
            return Err(String::from("The string is empty"));
        }
        return Ok(res);
    }
    pub fn eval_rpn(tokens: Vec<&str>) -> Result<i32, String> {
        let res = tokens.into_iter().fold(Vec::new(), |mut acc, x| {
            if x == "+" {
                let a = acc.pop().unwrap() as i32;
                let b = acc.pop().unwrap() as i32;
                acc.push(a + b);
                return acc;
            } else if x == "-" {
                let a = acc.pop().unwrap() as i32;
                let b = acc.pop().unwrap() as i32;
                acc.push(b - a);
                return acc;
            } else if x == "*" {
                let a = acc.pop().unwrap() as i32;
                let b = acc.pop().unwrap() as i32;
                acc.push(a * b);
                return acc;
            } else if x == "/" {
                let a = acc.pop().unwrap() as i32;
                let b = acc.pop().unwrap() as i32;
                acc.push(b / a);
                return acc;
            } else {
                acc.push(i32::from_str_radix(x, 10).unwrap());
                return acc;
            }
        });
        if res.len() > 0 {
            return Ok(res[0]);
        }
        return Err(String::from("The string is empty"));
    }
}
