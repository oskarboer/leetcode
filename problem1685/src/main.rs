impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        // prefix sum:
        let n = nums.len();
        let pref = nums.iter().scan(0, |acc, x| {
            *acc += *x;
            Some(*acc)
        }).collect::<Vec<i32>>();
        // postfix sum:
        let post = nums.iter().rev().scan(0, |acc, x| {
            *acc += *x;
            Some(*acc)
        }).collect::<Vec<i32>>().into_iter().rev().collect::<Vec<i32>>();


        let mut res = vec![0; nums.len()];

        // from prefix and postfix sums its easy to calculate the result: 
        // for prefix we multiply number by the number of prefix elements and subtract prefix from it 
        // for postfix we reverse it since postfix is larger.
        for i in 0..n {
            res[i] = (nums[i] * (i + 1) as i32 - pref[i]) + (post[i] - nums[i] * (n - i) as i32)
        }
    
        res
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_sum_absolute_differences(vec![2,3,5]), [4,3,5]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_sum_absolute_differences(vec![1,4,6,8,10]), [24,15,13,15,21]);
    }
}


struct Solution{}
fn main() {
    println!("{:?}", Solution::get_sum_absolute_differences(vec![2,3,5]));
}
