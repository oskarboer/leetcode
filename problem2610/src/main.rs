impl Solution {
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        nums.sort_unstable();
        let mut ret = vec![vec![]];

        let mut row = 0;
        let mut i = 1;
        let mut prev = nums[0];
        ret[0].push(nums[0]);
        while i < n {
            if nums[i] == prev {
                row += 1;
                if row >= ret.len() {
                    ret.push(vec![]);
                }
            } else {
                row = 0;
            }

            ret[row].push(nums[i]);
            prev = nums[i];
            i += 1;
        }
        ret
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::find_matrix(vec![1,3,4,1,2,3,1]));
}
