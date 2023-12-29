impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let n = job_difficulty.len();
        if n < d {
            return -1;
        }

        let mut dp = vec![vec![-1; n]; d];

        let mut max = job_difficulty[0];
        for i in 0..n {
            if max < job_difficulty[i] {
                max = job_difficulty[i];
            }
            dp[0][i] = max;
        }

        let mut sum = 0;
        for i in 0..(n.min(d)) {
            sum += job_difficulty[i];
            dp[i][i] = sum;
        }



        for day in 1..d {
            for task in (day + 1)..n {
                // search for range in what to look for min:
                let mut prev_big = task - 1;
                for i in (0..=(task-1)).rev() {
                    if job_difficulty[task] >= job_difficulty[i] {
                        prev_big = i;
                    } else {
                        break;
                    }
                }
                let min_day_before = dp[day - 1][(prev_big.max(day - 1))..=(task-1)].iter().fold(i32::MAX - 1000000, |a, e| a.min(*e)) + job_difficulty[task];



                // search for big in the same day
                let mut min_same_day = i32::MAX;
                let mut prev_larger = task;
                for i in (day..=(task-1)).rev() {
                    if job_difficulty[prev_larger] <= job_difficulty[i]{
                        prev_larger = i;
                        min_same_day = min_same_day.min(dp[day][i]);
                    }
                }



                dp[day][task] = min_day_before.min(min_same_day);

                println!("day: {day}, task: {task}, min same {min_same_day}, min before {min_day_before}");

            }
        }

        for i in &dp {
            println!("{:?}", i);
        }

        dp[d-1][n-1]
    }
}




#[cfg(test)]
mod test {
    use super::Solution;
    

    #[test]
    fn test1() {
        assert_eq!(Solution::min_difficulty(vec![1, 3, 5, 2, 3, 1, 8], 2), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_difficulty(vec![6,5,4,3,2,1], 2), 7);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_difficulty(vec![6,5,4], 4), -1);
    }


    #[test]
    fn test4() {
        assert_eq!(Solution::min_difficulty(vec![1, 3, 5, 2, 3, 1, 8], 3), 12);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::min_difficulty(vec![10, 3, 5, 1, 5, 3, 8], 3), 19);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::min_difficulty(vec![186, 398, 479, 206, 885, 423, 805, 112, 925, 656, 16, 932, 740, 292, 671, 360], 4), 1803);
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::min_difficulty(vec![3, 7, 9, 4, 14, 8, 13, 2, 15, 10, 1, 16, 12, 5, 11, 6], 4), 32);
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::min_difficulty(vec![10, 1, 16, 12, 5, 11, 6], 4), 33);
    }
}


struct Solution {}
fn main() {
    println!("Hello, world!");
}
