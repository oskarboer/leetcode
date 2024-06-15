use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, mut profits: Vec<i32>, mut capital: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut pandc: Vec<(i32, i32)> = profits.into_iter().zip(capital.into_iter()).collect(); 
        pandc.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        
        let mut available_projects = BinaryHeap::new();

        let mut i: usize = 0;

        while i < pandc.len() && w >= pandc[i].1 {
            available_projects.push(pandc[i].0);
            i += 1;
        }
        
        for _ in 0..k {
            w += match available_projects.pop() {
                None => return w,
                Some(value) => value,
            };
            
            while i < pandc.len() && w >= pandc[i].1 {
                available_projects.push(pandc[i].0);
                i += 1;
            }
        }

        w
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]), 4);
    }
}

fn main() {
    
}


