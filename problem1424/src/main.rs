impl Solution {
    // Here I create a diagonaly ordered matrix and pair every number with
    // its order number (enable prints to see it) and sort the final list by order numbers.
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut a = vec![];

        let mut c1 = 1;
        for i in 0..nums.len() {
            c1 += i;
            let sub = &nums[i];
            let mut c2 = c1;
            for j in 0..sub.len() {
                // print!("{} ", c2);
                a.push((c2, sub[j]));
                c2 += (j) + (i + 2);
            }
            // println!("");
        }

        a.sort_unstable_by_key(|a| a.0);

        a.iter().map(|a| a.1).collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = vec![1, 4, 2, 7, 5, 3, 8, 6, 9];

        assert_eq!(Solution::find_diagonal_order(input), output);
    }

    #[test]
    fn test2() {
        let input = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16],
        ];
        let output = vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16];

        assert_eq!(Solution::find_diagonal_order(input), output);
    }
}

struct Solution {}

fn main() {
    Solution::find_diagonal_order(vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
    ]);
}
