use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        nums.iter().for_each(|i| {count.entry(i).and_modify(|c| *c += 1).or_insert(1);});
        if count.values().any(|c| *c == 1) {
            return -1;
        }
        count.values().fold(0, |a, &c| {
            match c % 3 {
                0 => a + c / 3,
                1 => a + c / 3 + 1,
                2 => a + c / 3 + 1,
                _ => panic!("should not happen since Z % 3 expected to be 0, 1, 2"),
            }
        })


        
    }
}


struct Solution{}

fn main() {
    println!("{}", Solution::min_operations(vec![2, 2,3,3,3,4,4,4,4,5,5,5,5,5,6,6,6,6,6,6,6]));
}
