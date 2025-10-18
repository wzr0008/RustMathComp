pub mod primary_set {
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
}
