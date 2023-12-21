impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        let mut ys: Vec<i32> = points.into_iter().map(|v| v[0]).collect();
        ys.sort_unstable();
        println!("{:?}", ys);
        ys.clone().into_iter().zip(ys.into_iter().skip(1)).map(|(a, b)| b-a).max().unwrap()
    }
}



struct Solution{}
fn main() {
    println!("expect 1 got: {}", Solution::max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]]));
}
