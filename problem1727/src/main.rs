impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut msize = 0;


        for i in 1..n {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 1 {
                    matrix[i][j] += matrix[i-1][j];
                }
            }
        }

        println!("{:?}", matrix);

        for i in 0..n {
            matrix[i].sort_unstable();
            for j in 0..matrix[i].len() {
                msize = msize.max(matrix[i][j] * (matrix[i].len() - j) as i32);
            }
        }

        msize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::largest_submatrix(vec![vec![0,0,1],vec![1,1,1], vec![1,0,1]]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::largest_submatrix(vec![vec![1,0,1,0,1]]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::largest_submatrix(vec![vec![1,1,0],vec![1,0,1]]), 2);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::largest_submatrix(vec![vec![0,0,1],vec![1,0,1], vec![1,1,1]]), 4);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::largest_submatrix(vec![vec![1],vec![1], vec![1]]), 3);
    }
}


struct Solution {}
fn main() {
    println!("{}", Solution::largest_submatrix(vec![vec![0,0,1],vec![1,0,1], vec![1,1,1]]));
}
