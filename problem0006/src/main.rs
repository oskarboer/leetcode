impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut nummer = (0..(num_rows * 2 - 2)).cycle();

        let mut res = s.chars().map(|c| (c, nummer.next().unwrap())).collect::<Vec<(char, i32)>>();
        res.sort_by_key(|e| e.1);
        res.iter().map(|(c, n)| c).collect::<String>()
    }
}




struct Solution {}
fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 3));
}
