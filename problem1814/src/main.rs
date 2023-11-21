use std::collections::HashMap;

fn rev(mut a: i32) -> i32 {
    let mut result = 0;
    while a != 0 {
        result = result * 10 + a % 10;
        a /= 10;
    }
    result
}

fn pair_combinations(num_elements: usize) -> usize {
    match num_elements {
        0 => 0,
        1 => 0,
        n => n * (n - 1) / 2,
    }
}

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let modulo = 1_000_000_007;
        let mut pairs: HashMap<i32, usize> = HashMap::new();

        for n in nums {
            let key = (n - rev(n));
            pairs.entry(key).and_modify(|a| *a += 1).or_insert(1);
        }
        let mut total: usize = 0;
        for &num_pairs in pairs.values() {
            total = (total + pair_combinations(num_pairs)) % modulo;
        }
        total as i32
    }
}


struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rev_basic() {
        assert_eq!(rev(0), 0);
        assert_eq!(rev(1), 1);
        assert_eq!(rev(11), 11);
        assert_eq!(rev(123), 321);
        assert_eq!(rev(120), 21);
        assert_eq!(rev(12001), 10021);
        assert_eq!(rev(1234567890), 987654321);
    }

    #[test]
    fn pair_combinations_basic() {
        assert_eq!(pair_combinations(3), 3);
        assert_eq!(pair_combinations(10), 45);
        assert_eq!(pair_combinations(1), 0);
    }

    #[test]
    fn solution_test1() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
    }

    #[test]
    fn solution_test2() {
        assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
    }

    #[test]
    fn solution_test3() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 97]), 1);
    }

    #[test]
    fn solution_test4() {
        assert_eq!(Solution::count_nice_pairs(vec![42]), 0);
    }

    #[test]
    fn solution_test5() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 97, 97, 97]), 6);
    }

    #[test]
    fn solution_test6() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 24]), 0);
    }
}

fn main() {
    println!("First test should return 2: it returns: {}", Solution::count_nice_pairs(vec![42, 11, 1, 97]));
}
