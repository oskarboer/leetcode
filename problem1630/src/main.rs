impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let n = l.len();
        let mut res = vec![true; n];
        
        for i in 0..n {
            let (a, b) = (l[i] as usize, r[i] as usize);
            let range = a..=b;
            let mut sub = vec![0; b - a + 1];
            sub.clone_from_slice(&nums[range]);
            sub.sort_unstable();
            if sub.len() >= 2 {
                let tmp = sub[1] - sub[0];
                let mut prev = sub[1];
                for j in 2..sub.len() {
                    if sub[j] - prev != tmp {
                        res[i] = false;
                        break;
                    }
                    prev = sub[j];
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let output = vec![true, false, true];

        assert_eq!(Solution::check_arithmetic_subarrays(vec![4,6,5,9,3,7], vec![0,0,2], vec![2,3,5]), output);
    }

    #[test]
    fn test2() {
        let nums = vec![-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10];
        let l = vec![0,1,6,4,8,7];
        let r = vec![4,4,9,7,9,10];

        let output = vec![false,true,false,false,true,true];

        assert_eq!(Solution::check_arithmetic_subarrays(nums, l, r), output);
    }
}


struct Solution {}
fn main() {
    println!("{:?}", Solution::check_arithmetic_subarrays(vec![4,6,5,9,3,7], vec![0,0,2], vec![2,3,5]));
}
