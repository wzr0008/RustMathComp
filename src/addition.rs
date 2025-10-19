#[allow(dead_code)]
pub mod primary_set {
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
}
