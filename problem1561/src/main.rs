impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();

        // let n = piles.len();
        // let mut res = 0;
        // for i in 0..(n / 3) {
        //     res += piles[n - 2 * i - 2];
        // }
        // res

        // equivalent solution but in thery should be faster:
        let n = piles.len() / 3;
        piles.into_iter().skip(n).step_by(2).sum()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_coins(vec![2,4,1,2,7,8]), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_coins(vec![2,4,5]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_coins(vec![9,8,7,6,5,1,2,3,4]), 18);
    }
}

struct Solution {}
fn main() {
    println!("{}", Solution::max_coins(vec![2,4,1,2,7,8]));
}
