impl Solution {
    fn len(s: &Vec<char>) -> usize {
        let mut p_c = s[0];
        let mut count: usize = 1;
        let mut len: usize = 0;
        let mut total_len = 0;

        for &c in s {
            if p_c == c {
                count += 1;
                len = 1 + count.ilog(10) as usize + 1; // log + 1 (since we have at least one number and log starts with 0)
            } else {
                total_len += len;
                count = 1;
                p_c = c;
                len = 1
            }
        }
        total_len + len
    }

    fn rec(s: &Vec<char>, curr: Vec<char>, k: usize, i: usize) -> usize {
        if s.len() == i {
            return Solution::len(&curr);
        }
        let mut one = curr.clone();
        one.push(s[i]);

        let ret_one = Solution::rec(&s, one, k, i+1);

        if k > 0 {
            return Solution::rec(&s, curr, k-1, i+1).min(ret_one);
        }
        ret_one
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();

        return Solution::rec(&s, vec![], k as usize, 0) as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::get_length_of_optimal_compression(String::from("aaabcccd"), 2), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::get_length_of_optimal_compression(String::from("aabbaa"), 2), 2);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::get_length_of_optimal_compression(String::from("aaaaaaaaaaa"), 0), 3);
    }


    #[test]
    fn test_len_1() {
        let test: Vec<char> = String::from("aaaabbaaaaaaaaaa").chars().collect();
        assert_eq!(Solution::len(&test), 7);
    }
}

struct Solution {}

fn main() {
    // println!("{:?}", Solution::get_length_of_optimal_compression("ohoohhoooohhohoooohhhohoohhoohooohhhoohhhooohohhooohhoohhhhhhhooooohhoooohooohooohhohhhhhhohohoohhoo".to_string(), 74));     
    // println!("{:?}", Solution::get_length_of_optimal_compression("earmaypeacebeonearthmerrychristmashohohoooohappyhanukkahhappyholidayshappyholidaysandahappynewyearma".to_string(), 20));
    println!("{:?}", Solution::get_length_of_optimal_compression("eoongjjkjfelnkgkjohfjfjfhkmnmmlinkihhlfipgoejiniol".to_string(), 13));

}
