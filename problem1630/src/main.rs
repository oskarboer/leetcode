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





struct Solution {}
fn main() {
    println!("{:?}", Solution::check_arithmetic_subarrays(vec![4,6,5,9,3,7], vec![0,0,2], vec![2,3,5]));
}
