impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut nummer = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();

        // println!("{:?}", nummer.clone().take(10).collect::<Vec<_>>());

        let mut res = s
            .chars()
            .map(|c| (c, nummer.next().unwrap()))
            .collect::<Vec<(char, i32)>>();
        res.sort_by_key(|e| e.1);
        res.iter().map(|(c, _)| c).collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }
}

struct Solution {}
fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 3));
}
